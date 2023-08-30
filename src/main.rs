use indicatif::{ProgressBar, ProgressStyle};
use std::{
    io,
    path::Path,
    fs::{self, File},
};
use rayon::prelude::*;
use tar::Archive;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    let app = clap::App::new("extract")
        .arg(
            clap::Arg::with_name("files")
                .required(true)
                .multiple(true)
                .help("Files to be extracted"),
        );

    let matches = app.get_matches();
    let files: Vec<_> = matches.values_of("files").unwrap().collect();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?,
    );

    files.par_iter().for_each(|file| {
        if let Err(err) = extract_file(file, "output_directory/") {
            println!("Error extracting file '{}': {}", file, err);
        }
        pb.inc(1);
    });

    pb.finish_with_message("All files extracted successfully!");
    Ok(())
}
fn extract_tar(file: &str, output_dir: &str) -> color_eyre::eyre::Result<()> {
    let tar_file = File::open(file)?;
    let mut a = Archive::new(tar_file);

    for i in a.entries()? {
        let mut i = i?;
        let entry_path = i.header().path()?;
        let full_path = Path::new(output_dir).join(entry_path);

        if i.header().entry_type().is_dir() {
            fs::create_dir_all(&full_path)?;
        } else {
            fs::create_dir_all(&full_path.parent().unwrap())?;

            let mut file = File::create(&full_path)?;
            io::copy(&mut i, &mut file)?;
        }
    }

    Ok(())
}

fn extract_file(file: &str, output_dir: &str) -> color_eyre::eyre::Result<()> {
    if !Path::new(file).exists() {
        return Err(color_eyre::eyre::eyre!("'{}' - file does not exist", file));
    }
    if let Some("tar") = Path::new(file).extension().and_then(|s| s.to_str()) {
        extract_tar(file, output_dir)?;
    } else {
        println!("extract: '{}' - unknown archive method", file);
    }
    Ok(())
}