/*
TODO_2: implement remaining extractor functions and write tests
*/
use std::{fs::{self, File}, error::Error, io::{self, ErrorKind, Write, Read}, path::Path,};
use lzma::reader::LzmaReader;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use unrar::Archive;

pub fn extract_zip(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let file = fs::File::open(input_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let full_outpath = output_directory.join(&outpath);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&full_outpath).unwrap();
        } else {
            if let Some(p) = full_outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&full_outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&full_outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}

pub fn extract_rar(input_path: &Path, output_directory: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = Archive::new(input_path)
            .open_for_processing()
            .unwrap();
    while let Some(header) = archive.read_header()? {
        let output_path = output_directory.join(&header.entry().filename);
        archive = if header.entry().is_file() {
            header.extract_to(&output_path)?
        } else {
            header.skip()?
        };
    }
    Ok(())
}

pub fn extract_tar(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let tar_file = fs::File::open(input_path)?;
    let mut a = tar::Archive::new(tar_file);

    for i in a.entries()? {
        let mut i = i?;
        let entry_path = i.header().path()?;
        let full_path = output_directory.join(entry_path);

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

pub fn extract_lzma(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let mut decompressor = LzmaReader::new_decompressor(input_file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let output_file_path = output_directory.join(input_path.file_stem().unwrap());
    let mut output_file = File::create(&output_file_path)?;
    io::copy(&mut decompressor, &mut output_file)?;
    Ok(())
}

pub fn extract_gz(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let mut decompressor = GzDecoder::new(input_file);
    let output_file_path = output_directory.join(input_path.file_stem().unwrap());
    let mut output_file = File::create(&output_file_path)?;
    match io::copy(&mut decompressor, &mut output_file) {
        Ok(_) => Ok(()),
        Err(err) => Err(io::Error::new(ErrorKind::Other, err.to_string())),
    }
}

pub fn extract_bz2(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let bz2_reader = BzDecoder::new(input_file);
    let output_file_path = output_directory.join(input_path.file_stem().unwrap());
    let mut output_file = File::create(&output_file_path)?;
    let mut buffer = Vec::new();
    let mut decompressor = io::BufReader::new(bz2_reader);
    loop {
        let bytes_read = decompressor.read_to_end(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        output_file.write_all(&buffer[..bytes_read])?;
        buffer.clear();
    }

    Ok(())
}
pub fn extract_7z(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    sevenz_rust::decompress_file(input_path, output_directory).expect("complete");
    Ok(())
}
pub fn extract_tbz2(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let bz2_reader = BzDecoder::new(input_file);
    let mut archive = tar::Archive::new(bz2_reader);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let full_path = output_directory.join(entry_path);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&full_path)?;
        } else {
            fs::create_dir_all(&full_path.parent().unwrap())?;

            let mut file = fs::File::create(&full_path)?;
            io::copy(&mut entry, &mut file)?;
        }
    }
    Ok(())
}

pub fn extract_tgz(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let mut decompressor = GzDecoder::new(input_file);
    let mut archive = tar::Archive::new(&mut decompressor);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let full_path = output_directory.join(entry_path);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&full_path)?;
        } else {
            fs::create_dir_all(&full_path.parent().unwrap())?;
            let mut file = fs::File::create(&full_path)?;
            io::copy(&mut entry, &mut file)?;
        }
    }
    Ok(())
}

pub fn extract_txz(input_path: &Path, output_directory: &Path) -> Result<(), io::Error> {
    let input_file = File::open(input_path)?;
    let mut decompressor = LzmaReader::new_decompressor(input_file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let mut archive = tar::Archive::new(&mut decompressor);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let full_path = output_directory.join(entry_path);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&full_path)?;
        } else {
            fs::create_dir_all(&full_path.parent().unwrap())?;
            let mut file = fs::File::create(&full_path)?;
            io::copy(&mut entry, &mut file)?;
        }
    }
    Ok(())
}

    // pub fn extract_arj(){}
    // pub fn extract_cab(){}
    // pub fn extract_chm(){}
    // pub fn extract_deb(){}
    // pub fn extract_dmg(){}
    // pub fn extract_iso(){}
    // pub fn extract_lzh(){}
    // pub fn extract_msi(){}
    // pub fn extract_rpm(){}
    // pub fn extract_udf(){}
    // pub fn extract_wim(){}
    // pub fn extract_xar(){}
    // pub fn extract_exe(){}
