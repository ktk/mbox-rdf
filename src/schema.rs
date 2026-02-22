use crate::vocab::*;
use std::fs::File;
use std::io::Write;
use anyhow::Result;

pub fn generate_schema(vocab: &Vocab, output_path: &str) -> Result<()> {
    let mut f = File::create(output_path)?;
    let mail = &vocab.schema_base;
    
    writeln!(f, "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .")?;
    writeln!(f, "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .")?;
    writeln!(f, "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .")?;
    writeln!(f, "@prefix mail: <{}> .", mail)?;
    writeln!(f)?;

    // Classes
    write_class(&mut f, mail, "Message", "An email message")?;
    write_class(&mut f, mail, "Mailbox", "A mailbox or folder containing messages")?;
    write_class(&mut f, mail, "Address", "An email address (optionally with a display name)")?;
    write_class(&mut f, mail, "Attachment", "An email attachment metadata")?;
    write_class(&mut f, mail, "MailingList", "A mailing list or discussion group")?;

    // Properties
    write_property(&mut f, mail, "messageId", "Message", "xsd:string", "RFC 822 Message-ID")?;
    write_property(&mut f, mail, "mid", "Message", "xsd:anyURI", "Thunderbird-compatible mid: link")?;
    write_property(&mut f, mail, "subject", "Message", "xsd:string", "Message subject")?;
    write_property(&mut f, mail, "normalizedSubject", "Message", "xsd:string", "Message subject with prefixes like Re:/Fwd: removed")?;
    write_property(&mut f, mail, "userAgent", "Message", "xsd:string", "Mail client software used to send the message")?;
    write_property(&mut f, mail, "linksTo", "Message", "xsd:anyURI", "URL mentioned in the message body")?;
    write_property(&mut f, mail, "date", "Message", "xsd:dateTime", "Email sent date")?;
    write_property(&mut f, mail, "from", "Message", "mail:Address", "Sender")?;
    write_property(&mut f, mail, "to", "Message", "mail:Address", "Primary recipient")?;
    write_property(&mut f, mail, "cc", "Message", "mail:Address", "Carbon copy recipient")?;
    write_property(&mut f, mail, "bcc", "Message", "mail:Address", "Blind carbon copy recipient")?;
    write_property(&mut f, mail, "bodyText", "Message", "xsd:string", "Textual body content")?;
    write_property(&mut f, mail, "folder", "Message", "mail:Mailbox", "Folder containing the message")?;
    write_property(&mut f, mail, "sourcePath", "Message", "xsd:string", "Local path to the source mbox file")?;
    write_property(&mut f, mail, "hasAttachment", "Message", "mail:Attachment", "Link to attachment")?;
    write_property(&mut f, mail, "inReplyTo", "Message", "mail:Message", "Parent message this is a reply to")?;
    write_property(&mut f, mail, "references", "Message", "mail:Message", "Ancestor messages in the same thread")?;
    write_property(&mut f, mail, "belongsToList", "Message", "mail:MailingList", "The mailing list this message belongs to")?;
    write_property(&mut f, mail, "addr", "Address", "xsd:string", "Email address string")?;
    write_property(&mut f, mail, "name", "Address", "xsd:string", "Display name")?;
    write_property(&mut f, mail, "hash", "Attachment", "xsd:string", "SHA-256 hash of the content")?;
    write_property(&mut f, mail, "filename", "Attachment", "xsd:string", "Attachment filename")?;
    write_property(&mut f, mail, "contentType", "Attachment", "xsd:string", "MIME content type")?;
    write_property(&mut f, mail, "listId", "MailingList", "xsd:string", "Mailing list identifier")?;
    write_property(&mut f, mail, "listArchive", "MailingList", "xsd:anyURI", "URL of the mailing list archive")?;

    Ok(())
}

fn write_class(f: &mut File, _ns: &str, name: &str, label: &str) -> std::io::Result<()> {
    writeln!(f, "mail:{} rdf:type rdfs:Class ;", name)?;
    writeln!(f, "    rdfs:label \"{}\" .", label)?;
    writeln!(f)
}

fn write_property(f: &mut File, _ns: &str, name: &str, domain: &str, range: &str, label: &str) -> std::io::Result<()> {
    writeln!(f, "mail:{} rdf:type rdf:Property ;", name)?;
    writeln!(f, "    rdfs:label \"{}\" ;", label)?;
    writeln!(f, "    rdfs:domain mail:{} ;", domain)?;
    if range.contains(':') && !range.starts_with("mail:") && !range.starts_with("xsd:") {
        writeln!(f, "    rdfs:range <{}> .", range)?;
    } else {
        writeln!(f, "    rdfs:range {} .", range)?;
    }
    writeln!(f)
}
