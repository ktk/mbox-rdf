use clap::Args;
use std::path::Path;
use std::fs::{self, File};
use std::io::BufWriter;
use anyhow::{Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::process::Command;

use crate::config::MboxConfig;
use crate::vocab::{Vocab, DEFAULT_SCHEMA_IRI};
use crate::{process_mbox_file, AttachmentOpts};

#[derive(Args, Debug)]
pub struct ConvertArgs {
    /// Config file path
    #[arg(long, default_value = "mbox-config.toml")]
    pub config: String,

    /// Only convert this account
    #[arg(long)]
    pub account: Option<String>,

    /// Show what would be done without running
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Number of parallel conversions
    #[arg(long, default_value_t = 1)]
    pub parallel: usize,

    /// Don't rebuild QLever index after conversion
    #[arg(long, default_value_t = false)]
    pub skip_index: bool,
}

pub fn run(args: ConvertArgs) -> Result<()> {
    let config_content = fs::read_to_string(&args.config)
        .with_context(|| format!("Failed to read {}", args.config))?;
    
    let config: MboxConfig = toml::from_str(&config_content)
        .context("Failed to parse config TOML")?;

    let out_dir = Path::new(&config.settings.output_dir);
    if !args.dry_run {
        fs::create_dir_all(out_dir)
            .with_context(|| format!("Failed to create output dir {}", out_dir.display()))?;
    }

    let is_gzip = config.settings.compress;

    let mut total_accounts_processed = 0;
    let mut total_folders_processed = 0;

    for (acc_key, acc_config) in &config.accounts {
        if let Some(target_acc) = &args.account {
            if acc_key != target_acc && &acc_config.email != target_acc {
                continue;
            }
        }

        total_accounts_processed += 1;
        println!("\n▶️ Processing account: {} ({})", acc_config.email, acc_config.graph);

        let vocab = Vocab::new(DEFAULT_SCHEMA_IRI.to_string(), config.settings.data_iri.clone());

        for folder in &acc_config.folders {
            if !folder.include {
                continue;
            }

            // Replace / with _ for filenames
            let safe_folder = folder.name.replace('/', "_");
            let ext = if is_gzip { "nq.gz" } else { "nq" };
            let out_filename = format!("{}.{}.{}", acc_config.email, safe_folder, ext);
            let out_path = out_dir.join(&out_filename);
            let in_path = Path::new(&folder.path);

            if !in_path.exists() {
                println!("   ❌ Skipping {} (path not found: {})", folder.name, in_path.display());
                continue;
            }

            println!("   📂 Folder: {} -> {}", folder.name, out_filename);

            if args.dry_run {
                continue;
            }

            let out_file = File::create(&out_path)
                .with_context(|| format!("Failed to create output file {}", out_path.display()))?;
            
            let writer: Box<dyn std::io::Write> = if is_gzip {
                Box::new(GzEncoder::new(out_file, Compression::default()))
            } else {
                Box::new(out_file)
            };
            
            let mut buf_writer = BufWriter::new(writer);

            let att_opts = if acc_config.include_attachments {
                if let Some(ref dir) = acc_config.attachment_dir {
                    if !args.dry_run {
                        fs::create_dir_all(dir)
                            .with_context(|| format!("Failed to create attachment directory {}", dir))?;
                    }
                    Some(AttachmentOpts { max_size: acc_config.max_attachment_size, dir })
                } else {
                    println!("   ⚠️ include_attachments is true but no attachment_dir specified for {}", acc_config.email);
                    None
                }
            } else {
                None
            };

            match process_mbox_file(
                in_path,
                &folder.name,
                acc_config.limit, // limit
                acc_config.include_body, // include_body
                att_opts.as_ref(),
                Some(&acc_config.graph),
                &vocab,
                &mut buf_writer,
            ) {
                Ok((processed, failed)) => {
                    println!("      ✓ {} messages ({} failed)", processed, failed);
                    total_folders_processed += 1;
                }
                Err(e) => {
                    println!("      ❌ Error: {}", e);
                }
            }
        }
    }

    if args.dry_run {
        println!("\nDry run finished.");
        return Ok(());
    }

    println!("\n✅ Conversion complete: {} accounts, {} folders generated in {}", 
        total_accounts_processed, total_folders_processed, out_dir.display());

    if !args.skip_index {
        if let Some(qlever_dir) = &config.settings.qlever_dir {
            println!("\n🚀 Rebuilding QLever index in {}", qlever_dir);
            let q_dir = Path::new(qlever_dir);
            if q_dir.exists() && q_dir.join("Qleverfile").exists() {
                // cd qlever && qlever stop && qlever index --overwrite-existing && qlever start
                let _ = Command::new("qlever")
                    .arg("stop")
                    .current_dir(q_dir)
                    .status();
                
                let idx_status = Command::new("qlever")
                    .arg("index")
                    .arg("--overwrite-existing")
                    .current_dir(q_dir)
                    .status()?;
                
                if idx_status.success() {
                    Command::new("qlever")
                        .arg("start")
                        .current_dir(q_dir)
                        .status()?;
                    println!("QLever restarted successfully.");
                } else {
                    println!("QLever indexing failed.");
                }
            } else {
                println!("Directory {} doesn't exist or doesn't have a Qleverfile. Skipping index build.", qlever_dir);
            }
        }
    }

    Ok(())
}
