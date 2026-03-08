use clap::Args;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::{BTreeMap, HashMap};
use regex::Regex;
use anyhow::{Context, Result};
use crate::config::{MboxConfig, Settings, AccountConfig, FolderConfig};

#[derive(Args, Debug)]
pub struct DiscoverArgs {
    /// Override Thunderbird profile directory path
    #[arg(long)]
    pub profile_dir: Option<String>,

    /// Output config file path
    #[arg(short, long, default_value = "mbox-config.toml")]
    pub output: String,

    /// Include all folders by default (even Trash/Drafts)
    #[arg(long, default_value_t = false)]
    pub include_all: bool,
}

fn thunderbird_profiles_ini() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    { dirs::home_dir().map(|h| h.join("Library/Thunderbird/profiles.ini")) }
    
    #[cfg(target_os = "linux")]
    { dirs::home_dir().map(|h| h.join(".thunderbird/profiles.ini")) }
    
    #[cfg(target_os = "windows")]
    { dirs::config_dir().map(|h| h.join("Thunderbird/profiles.ini")) }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    { None }
}

fn get_profile_dir() -> Result<PathBuf> {
    let ini_path = thunderbird_profiles_ini()
        .context("Could not determine Thunderbird profiles.ini location for this OS")?;
    
    if !ini_path.exists() {
        anyhow::bail!("Thunderbird profiles.ini not found at {}", ini_path.display());
    }

    let contents = fs::read_to_string(&ini_path)?;
    let mut paths = Vec::new();
    let mut default_path = None;

    // Simple manual INI parse
    let mut current_path = None;
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            current_path = None;
        } else if let Some(path) = line.strip_prefix("Path=") {
            let p = path.to_string();
            current_path = Some(p.clone());
            paths.push(p);
        } else if let Some(path) = line.strip_prefix("Default=") {
            if path != "1" && path != "0" {
                default_path = Some(path.to_string());
            } else if path == "1" {
                if let Some(ref cp) = current_path {
                    default_path = Some(cp.clone());
                }
            }
        }
    }

    let base_dir = ini_path.parent().unwrap();
    
    // Check default path first
    if let Some(dp) = default_path {
        let dir = base_dir.join(&dp);
        if dir.join("prefs.js").exists() {
            return Ok(dir);
        }
    }

    // Fallback to any path that has prefs.js
    for p in paths {
        let dir = base_dir.join(&p);
        if dir.join("prefs.js").exists() {
            return Ok(dir);
        }
    }

    anyhow::bail!("No valid Thunderbird profile found (missing prefs.js) in {}", ini_path.display());
}

fn parse_prefs_js(profile_dir: &Path) -> Result<HashMap<String, String>> {
    let prefs_path = profile_dir.join("prefs.js");
    if !prefs_path.exists() {
        anyhow::bail!("prefs.js not found in profile dir: {}", profile_dir.display());
    }

    let content = fs::read_to_string(&prefs_path)?;
    
    // serverX -> directory name under ImapMail
    let mut server_dirs = HashMap::new();
    // serverX -> type (imap, pop3, etc)
    let mut server_types = HashMap::new();
    // accountX -> serverX
    let mut account_servers = HashMap::new();
    // accountX -> idX (comma separated, we take first)
    let mut account_identities = HashMap::new();
    // idX -> email
    let mut identity_emails = HashMap::new();

    let re_pref = Regex::new(r#"user_pref\("([^"]+)",\s*"([^"]+)"\);"#).unwrap();

    for cap in re_pref.captures_iter(&content) {
        let key = &cap[1];
        let val = &cap[2];

        if key.starts_with("mail.server.") {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() >= 4 {
                let server = parts[2];
                let prop = parts[3];
                if prop == "directory-rel" {
                    if let Some(dir) = val.strip_prefix("[ProfD]ImapMail/") {
                        server_dirs.insert(server.to_string(), dir.to_string());
                    }
                } else if prop == "type" {
                    server_types.insert(server.to_string(), val.to_string());
                }
            }
        } else if key.starts_with("mail.account.") {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() >= 4 {
                let account = parts[2];
                let prop = parts[3];
                if prop == "server" {
                    account_servers.insert(account.to_string(), val.to_string());
                } else if prop == "identities" {
                    let first_id = val.split(',').next().unwrap_or("").to_string();
                    if !first_id.is_empty() {
                        account_identities.insert(account.to_string(), first_id);
                    }
                }
            }
        } else if key.starts_with("mail.identity.") {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() >= 4 {
                let id = parts[2];
                let prop = parts[3];
                if prop == "useremail" {
                    identity_emails.insert(id.to_string(), val.to_string());
                }
            }
        }
    }

    // Map directory -> email
    let mut dir_to_email = HashMap::new();
    for (account, server) in account_servers {
        if let Some(stype) = server_types.get(&server) {
            if stype == "imap" || stype == "none" { 
                if let Some(dir) = server_dirs.get(&server) {
                    if let Some(id) = account_identities.get(&account) {
                        if let Some(email) = identity_emails.get(id) {
                            dir_to_email.insert(dir.clone(), email.clone());
                        }
                    }
                }
            }
        }
    }

    Ok(dir_to_email)
}

fn derive_folder_name(path: &Path, server_dir: &Path, dedup_re: &Regex) -> String {
    let rel = path.strip_prefix(server_dir).unwrap_or(path);
    rel.components()
        .filter(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            !s.ends_with(".sbd") && !s.ends_with(".msf")
        })
        .map(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            // Strip -N dedup suffix (e.g. Sent-1 -> Sent)
            dedup_re.replace(s, "").into_owned()
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn scan_folders(server_dir: &Path, dedup_re: &Regex) -> Result<Vec<(String, PathBuf, u64)>> {
    let mut folders = Vec::new();
    if !server_dir.exists() {
        return Ok(folders);
    }

    let mut dirs_to_visit = vec![server_dir.to_path_buf()];

    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let meta = entry.metadata()?;

                if meta.is_dir() {
                    let name = entry.file_name().into_string().unwrap_or_default();
                    if name.ends_with(".sbd") {
                        dirs_to_visit.push(path);
                    }
                } else if meta.is_file() {
                    let name = entry.file_name().into_string().unwrap_or_default();
                    if !name.ends_with(".msf") && !name.starts_with('.') && name != "msgFilterRules.dat" {
                        let folder_name = derive_folder_name(&path, server_dir, dedup_re);
                        folders.push((folder_name, path, meta.len()));
                    }
                }
            }
        }
    }

    // Sort alphabetically by folder name
    folders.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(folders)
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GiB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MiB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KiB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn run(args: DiscoverArgs) -> Result<()> {
    let profile_dir = if let Some(pd) = args.profile_dir {
        PathBuf::from(pd)
    } else {
        get_profile_dir()?
    };

    println!("🔍 Found Thunderbird profile: {}", profile_dir.display());

    let host_to_email = parse_prefs_js(&profile_dir)?;
    if host_to_email.is_empty() {
        println!("⚠️ No IMAP accounts found in prefs.js.");
        return Ok(());
    }

    let imap_mail_dir = profile_dir.join("ImapMail");
    if !imap_mail_dir.exists() {
        anyhow::bail!("ImapMail directory not found at {}", imap_mail_dir.display());
    }

    let mut config_accounts = BTreeMap::new();
    let dedup_re = Regex::new(r"-\d+$").unwrap();

    let mut total_accounts = 0;
    let mut total_folders = 0;
    let mut total_included = 0;

    for entry in fs::read_dir(&imap_mail_dir)?.flatten() {
        if entry.metadata()?.is_dir() {
            let host_dir = entry.file_name().into_string().unwrap_or_default();
            
            // For Smart mailboxes etc.
            if host_dir == "Local Folders" || host_dir.starts_with('.') {
                continue;
            }

            // Find matching email
            let email = host_to_email.get(&host_dir).cloned().unwrap_or_else(|| {
                format!("unknown@{}", host_dir)
            });

            // Account key based on LHS of email
            let account_key = email.split('@').next().unwrap_or(&email).to_string();

            println!("\n📧 {} ({})", email, host_dir);

            let mut mapped_folders = Vec::new();
            let folders = scan_folders(&entry.path(), &dedup_re)?;

            for (folder_name, path, size) in folders {
                if size == 0 {
                    continue; // Skip empty mbox files
                }

                // Subscription-style defaults
                let mut include = false;
                let ln = folder_name.to_lowercase();
                
                if args.include_all {
                    include = true;
                } else {
                    if ln == "inbox" || ln == "sent" || ln == "sent messages" || ln == "sent items" {
                        include = true;
                    }
                }

                total_folders += 1;
                if include {
                    total_included += 1;
                    println!("   ✅ {:25} {:>10}", folder_name, format_size(size));
                } else {
                    println!("   ⏭️  {:25} {:>10}  (excluded)", folder_name, format_size(size));
                }

                mapped_folders.push(FolderConfig {
                    name: folder_name,
                    path: path.to_string_lossy().to_string(),
                    include,
                });
            }

            if !mapped_folders.is_empty() {
                total_accounts += 1;
                config_accounts.insert(account_key, AccountConfig {
                    email: email.clone(),
                    graph: format!("urn:email:{}", email),
                    folders: mapped_folders,
                });
            }
        }
    }

    let config = MboxConfig {
        settings: Settings {
            output_dir: "qlever".to_string(),
            qlever_dir: Some("qlever".to_string()),
            compress: true,
        },
        accounts: config_accounts,
    };

    let toml_str = toml::to_string_pretty(&config)?;
    let mut out_content = String::new();
    out_content.push_str("# Auto-generated by `mbox-rdf discover`, edit as needed\n\n");
    out_content.push_str(&toml_str);

    fs::write(&args.output, out_content)
        .with_context(|| format!("Failed to write config to {}", args.output))?;

    println!("\nWritten: {} ({} accounts, {} folders, {} included)", 
        args.output, total_accounts, total_folders, total_included);
    println!("Edit the file to customize, then run: mbox-rdf convert");

    Ok(())
}
