use clap::{App, Arg};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::ExecutableCommand;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::stdout;
use std::path::Path;
use std::process::Command;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let app = App::new("extract")
    .arg(
        Arg::with_name("files")
            .required(true)
            .multiple(true)
            .help("Files to be extracted"),
        );

use rayon::prelude::*;
let matches = app.get_matches();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    files.par_iter().for_each(|file| {
        if let Err(err) = extract_file(file) {
            // You can handle the error here, perhaps by printing a message.
            println!("Error extracting file '{}': {}", file, err);
        }
        pb.inc(1);
    });


    pb.finish_with_message("All files extracted successfully!");
    Ok(())
}

fn extract_file(file: &str) -> color_eyre::eyre::Result<()> {
    if !Path::new(file).exists() {
        return Err(color_eyre::eyre::eyre!("'{}' - file does not exist", file));
    }

    let command = match Path::new(file).extension().and_then(|s| s.to_str()) {
        Some("tar") | Some("tar.gz") | Some("tar.xz") | Some("tbz2") | Some("tgz") | Some("txz") => "tar",
        Some("lzma") => "unlzma",
        Some("bz2") => "bunzip2",
        Some("rar") => "unrar",
        Some("gz") => "gunzip",
        Some("zip") => "unzip",
        Some("z") => "uncompress",
        Some("7z") | Some("arj") | Some("cab") | Some("chm") | Some("deb") | Some("dmg") | Some("iso") | Some("lzh") | Some("msi") | Some("rpm") | Some("udf") | Some("wim") | Some("xar") => "7z",
        Some("xz") => "unxz",
        Some("exe") => "cabextract",
        _ => {
            println!("extract: '{}' - unknown archive method", file);
            return Ok(());
        }
    };

    let args = match command {
        "tar" => vec!["xvf", file],
        "unrar" => vec!["x", "-ad", file],
        "7z" => vec!["x", file],
        _ => vec![file],
    };

    let output = Command::new(command).args(&args).output()?;

    if !output.status.success() {
        let mut stdout = stdout();
        stdout
            .execute(SetForegroundColor(Color::Red))?
            .execute(Print(format!(
                "Failed to extract '{}', command returned error\n",
                file
            )))?
            .execute(ResetColor)?;
        return Err(eyre::eyre!("Extraction failed for {}", file));
    }

    Ok(())
}
