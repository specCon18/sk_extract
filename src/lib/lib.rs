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
    // Helper function to create a temporary directory for testing
    fn create_temp_dir() -> PathBuf {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("test_dir");
        fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
        temp_dir
    }

    #[test]
    fn test_extract_zip() {
        let input_path = Path::new("test_data/test.zip");
        let output_directory = create_temp_dir();

        let result = extract_zip(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_rar() {
        let input_path = Path::new("test_data/test.rar");
        let output_directory = create_temp_dir();

        let result = extract_rar(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tar() {
        let input_path = Path::new("test_data/test.tar");
        let output_directory = create_temp_dir();

        let result = extract_tar(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_lzma() {
        let input_path = Path::new("test_data/test.lzma");
        let output_directory = create_temp_dir();

        let result = extract_lzma(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_gz() {
        let input_path = Path::new("test_data/test.gz");
        let output_directory = create_temp_dir();

        let result = extract_gz(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_bz2() {
        let input_path = Path::new("test_data/test.bz2");
        let output_directory = create_temp_dir();

        let result = extract_bz2(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_7z() {
        let input_path = Path::new("test_data/test.7z");
        let output_directory = create_temp_dir();

        let result = extract_7z(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tbz2() {
        let input_path = Path::new("test_data/test.tbz2");
        let output_directory = create_temp_dir();

        let result = extract_tbz2(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_tgz() {
        let input_path = Path::new("test_data/test.tgz");
        let output_directory = create_temp_dir();

        let result = extract_tgz(input_path, &output_directory);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_txz() {
        let input_path = Path::new("test_data/test.txz");
        let output_directory = create_temp_dir();

        let result = extract_txz(input_path, &output_directory);
        assert!(result.is_ok());
    }
}
