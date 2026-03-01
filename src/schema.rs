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
    writeln!(f, "@prefix schema: <{}> .", SCHEMA_ORG)?;
    writeln!(f, "@prefix mail: <{}> .", mail)?;
    writeln!(f)?;

    // Classes
    write_class(&mut f, mail, "Message", "An email message (also a schema:CreativeWork)")?;
    write_class(&mut f, mail, "Account", "An email account identity (identified by mailto: URI)")?;
    write_class(&mut f, mail, "Mailbox", "A mailbox or folder containing messages")?;
    write_class(&mut f, mail, "Thread", "A conversation thread grouping related messages")?;
    write_class(&mut f, mail, "MailingList", "A mailing list or discussion group")?;

    // Note: Attachments are typed as schema:MediaObject with schema.org properties
    writeln!(f, "# Attachments use schema:MediaObject with properties:")?;
    writeln!(f, "# schema:associatedMedia (Message -> schema:MediaObject)")?;
    writeln!(f, "# schema:name, schema:encodingFormat, schema:sha256, schema:contentSize")?;
    writeln!(f)?;
    // Note: Dates use schema.org properties
    writeln!(f, "# Dates use schema:dateCreated and schema:dateReceived")?;
    writeln!(f, "# Display names use schema:name")?;
    writeln!(f)?;

    // Message properties
    write_property(&mut f, mail, "messageId", "Message", "xsd:string", "RFC 822 Message-ID")?;
    write_property(&mut f, mail, "mid", "Message", "xsd:anyURI", "Thunderbird-compatible mid: link")?;
    write_property(&mut f, mail, "subject", "Message", "xsd:string", "Message subject")?;
    write_property(&mut f, mail, "normalizedSubject", "Message", "xsd:string", "Message subject with prefixes like Re:/Fwd: removed")?;
    write_property(&mut f, mail, "userAgent", "Message", "xsd:string", "Mail client software used to send the message")?;
    write_property(&mut f, mail, "linksTo", "Message", "xsd:anyURI", "URL mentioned in the message body")?;
    write_property(&mut f, mail, "from", "Message", "mail:Account", "Sender")?;
    write_property(&mut f, mail, "to", "Message", "mail:Account", "Primary recipient")?;
    write_property(&mut f, mail, "cc", "Message", "mail:Account", "Carbon copy recipient")?;
    write_property(&mut f, mail, "bcc", "Message", "mail:Account", "Blind carbon copy recipient")?;
    write_property(&mut f, mail, "replyTo", "Message", "mail:Account", "Reply-To address")?;
    write_property(&mut f, mail, "bodyText", "Message", "xsd:string", "Textual body content")?;
    write_property(&mut f, mail, "folder", "Message", "mail:Mailbox", "Folder containing the message")?;
    write_property(&mut f, mail, "sourcePath", "Message", "xsd:string", "Local path to the source mbox file")?;
    write_property(&mut f, mail, "inReplyTo", "Message", "mail:Message", "Parent message this is a reply to")?;
    write_property(&mut f, mail, "references", "Message", "mail:Message", "Ancestor messages in the same thread")?;
    write_property(&mut f, mail, "thread", "Message", "mail:Thread", "Conversation thread this message belongs to")?;
    write_property(&mut f, mail, "belongsToList", "Message", "mail:MailingList", "The mailing list this message belongs to")?;
    write_property(&mut f, mail, "attachmentCount", "Message", "xsd:integer", "Number of attachments")?;
    write_property(&mut f, mail, "size", "Message", "xsd:integer", "Raw message size in bytes")?;

    // MailingList properties
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
