use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::{fs::{self, File},io::{BufReader, Read},os::unix::fs::PermissionsExt,path::Path};

pub mod extractors;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::path::Path;
use extractors::{
    extract_zip,
    extract_rar,
    extract_tar,
    extract_lzma,
    extract_bz2,
    extract_tbz2,
    extract_tgz,
    extract_txz,
    extract_gz,
    extract_7z,
    // extract_arj,
    // extract_cab,
    // extract_chm,
    // extract_deb,
    // extract_dmg,
    // extract_iso,
    // extract_lzh,
    // extract_msi,
    // extract_rpm,
    // extract_udf,
    // extract_wim,
    // extract_xar,    
    // extract_exe   
};

    #[test]
    fn test_extract_zip() {
        let input_path = Path::new("src/test_data/test.zip");
        let output_directory = create_temp_dir();

        let result = extract_zip(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_rar() {
        let input_path = Path::new("src/test_data/test.rar");
        let output_directory = create_temp_dir();

        let result = extract_rar(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tar() {
        let input_path = Path::new("src/test_data/test.tar");
        let output_directory = create_temp_dir();

        let result = extract_tar(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_lzma() {
        let input_path = Path::new("src/test_data/test.lzma");
        let output_directory = create_temp_dir();

        let result = extract_lzma(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_gz() {
        let input_path = Path::new("src/test_data/test.gz");
        let output_directory = create_temp_dir();

        let result = extract_gz(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_bz2() {
        let input_path = Path::new("src/test_data/test.bz2");
        let output_directory = create_temp_dir();

        let result = extract_bz2(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_7z() {
        let input_path = Path::new("src/test_data/test.7z");
        let output_directory = create_temp_dir();

        let result = extract_7z(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tbz2() {
        let input_path = Path::new("src/test_data/test.tbz2");
        let output_directory = create_temp_dir();

        let result = extract_tbz2(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tgz() {
        let input_path = Path::new("src/test_data/test.tgz");
        let output_directory = create_temp_dir();

        let result = extract_tgz(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_txz() {
        let input_path = Path::new("src/test_data/test.txz");
        let output_directory = create_temp_dir();

        let result = extract_txz(input_path, &output_directory);
        assert!(result.is_ok());
    }

    fn create_temp_dir() -> PathBuf {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("test_dir");
        fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
        temp_dir
    }
    
    fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 1024];
    
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }
    
        Ok(context.finish())
    }
    fn mode_to_chmod(mode: u32) -> u32 {
        let mut flags:u32 = 0;
        
        // Owner permissions
        if (mode & 0o400) != 0 { flags = flags+400 } else { flags = flags+0 };
        if (mode & 0o200) != 0 { flags = flags+200 } else { flags = flags+0 };
        if (mode & 0o100) != 0 { flags = flags+100 } else { flags = flags+0 };
    
        // Group permissions
        if (mode & 0o40) != 0 { flags = flags+40 } else { flags = flags+0 };
        if (mode & 0o20) != 0 { flags = flags+20 } else { flags = flags+0 };
        if (mode & 0o10) != 0 { flags = flags+10 } else { flags = flags+0 };
    
        // Others permissions
        if (mode & 0o4) != 0 { flags = flags+4 } else { flags = flags+0 };
        if (mode & 0o2) != 0 { flags = flags+2 } else { flags = flags+0 };
        if (mode & 0o1) != 0 { flags = flags+1 } else { flags = flags+0 };
    
        flags
    }
}