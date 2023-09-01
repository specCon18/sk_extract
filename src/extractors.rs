use std::{fs, io, path::Path};
use unrar::Archive;
use xz2::bufread::XzDecoder;

// pub fn extract_xz(xz_file: &Path) -> io::Result<()> {
//     let file = fs::File::open(xz_file)?;
//     let decompressor = XzDecoder::new(file);
//     let mut archive = tar::Archive::new(decompressor);

//     for entry in archive.entries()? {
//         let mut entry = entry?;

//         let entry_path = entry.path()?;
//         let full_path = Path::new("output_directory/").join(entry_path);

//         if entry.header().entry_type().is_dir() {
//             fs::create_dir_all(&full_path)?;
//         } else {
//             fs::create_dir_all(&full_path.parent().unwrap())?;

//             let mut file = fs::File::create(&full_path)?;
//             io::copy(&mut entry, &mut file)?;
//         }
//     }

//     Ok(())
// }

pub fn extract_zip(zip_file: &Path) -> io::Result<()> {
    let file = fs::File::open(zip_file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}

pub fn extract_rar(rar_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = Archive::new(rar_file)
        .open_for_processing()
        .unwrap();

    while let Some(header) = archive.read_header()? {
        println!(
            "{} bytes: {}",
            header.entry().unpacked_size,
            header.entry().filename.to_string_lossy(),
        );
        archive = if header.entry().is_file() {
            header.extract()?
        } else {
            header.skip()?
        };
    }

    Ok(())
}

 pub fn extract_tar(tar_file: &Path) -> io::Result<()> {
    let tar_file = fs::File::open(tar_file)?;
    let mut a = tar::Archive::new(tar_file);

    for i in a.entries()? {
        let mut i = i?;
        let entry_path = i.header().path()?;
        let full_path = Path::new("output_directory/").join(entry_path);

        if i.header().entry_type().is_dir() {
            fs::create_dir_all(&full_path)?;
        } else {
            fs::create_dir_all(&full_path.parent().unwrap())?;

            let mut file = fs::File::create(&full_path)?;
            io::copy(&mut i, &mut file)?;
        }
    }

    Ok(())
}
