
pub mod extractors;

#[cfg(test)]
mod tests {
    use std::{io,os::unix::fs::PermissionsExt};
    use serial_test::serial;
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::path::Path;
    use std::io::Read;
use extractors::{
    extract_zip,
    extract_rar,
    extract_tar,
    extract_tbz2,
    extract_tgz,
    extract_txz,
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
#[serial]
fn test_extract_zip() {
    let input_path = Path::new("src/test_data/test.zip");
    let output_directory = create_temp_dir();

    // Extract the zip file
    let result = extract_zip(input_path, &output_directory);
    assert!(result.is_ok());

    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());
    
    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_rar() {
    let input_path = Path::new("src/test_data/test.rar");
    let output_directory = create_temp_dir();
    let result = extract_rar(input_path, &output_directory);
    assert!(result.is_ok());
    
    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());

    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_tar() {
    let input_path = Path::new("src/test_data/test.tar");
    let output_directory = create_temp_dir();
    let result = extract_tar(input_path, &output_directory);
    assert!(result.is_ok());
    
    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());
    
    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_7z() {
    let input_path = Path::new("src/test_data/test.7z");
    let output_directory = create_temp_dir();
    let result = extract_7z(input_path, &output_directory);
    assert!(result.is_ok());
    
    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());

    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_tbz2() {
    let input_path = Path::new("src/test_data/test.tbz2");
    let output_directory = create_temp_dir();

    let result = extract_tbz2(input_path, &output_directory);
    assert!(result.is_ok());

    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());
    
    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_tgz() {
    let input_path = Path::new("src/test_data/test.tgz");
    let output_directory = create_temp_dir();
    let result = extract_tgz(input_path, &output_directory);
    assert!(result.is_ok());

    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());

    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

#[test]
#[serial]
fn test_extract_txz() {
    let input_path = Path::new("src/test_data/test.txz");
    let output_directory = create_temp_dir();
    let result = extract_txz(input_path, &output_directory);
    assert!(result.is_ok());
    
    // Check checksums and assert equality
    let checksum_01 = verify_checksum("test_dir/checksum_01", "test_dir/testfile_01").unwrap();
    assert_eq!(checksum_01, true);
    
    let checksum_02 = verify_checksum("test_dir/checksum_02", "test_dir/testfile_02").unwrap();
    assert_eq!(checksum_02, true);
    
    let checksum_03 = verify_checksum("test_dir/checksum_03", "test_dir/testfile_03").unwrap();
    assert_eq!(checksum_03, true);

    // Check Permissions match original 644 compression perms
    let tf1_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf1_perms.is_ok());

    let tf2_perms = check_permissions("test_dir/testfile_02",644);
    assert!(tf2_perms.is_ok());
    
    let tf3_perms = check_permissions("test_dir/testfile_01",644);
    assert!(tf3_perms.is_ok());

    let csum1_perms = check_permissions("test_dir/checksum_01",644);
    assert!(csum1_perms.is_ok());

    let csum2_perms = check_permissions("test_dir/checksum_02",644);
    assert!(csum2_perms.is_ok());

    let csum3_perms = check_permissions("test_dir/checksum_03",644);
    assert!(csum3_perms.is_ok());

    // Delete the test directory at the end of the test
    if let Err(err) = fs::remove_dir_all(&output_directory) {
        eprintln!("Failed to delete test directory: {:?}", err);
    } 
}

fn verify_checksum(checksum_path: &str, testfile_path: &str) -> Result<bool, std::io::Error> {
    let mut checksum_file = fs::File::open(checksum_path).expect("Failed to open checksum file");
    let mut checksum_data = String::new();
    checksum_file
        .read_to_string(&mut checksum_data)
        .expect("Failed to read checksum data");
    let mut checksum_data_uppercase = checksum_data.to_uppercase();
    
    if checksum_data_uppercase.len() >= 2 {
        checksum_data_uppercase.truncate(checksum_data_uppercase.len() - 1);
    } else {
        eprintln!("String is too short to remove characters");
    }

    let mut testfile = fs::File::open(testfile_path).expect("Failed to open test file");
    let testfile_buffer = io::BufReader::new(&mut testfile);
    let calculated_checksum = data_encoding::HEXUPPER.encode(sha256_digest(testfile_buffer)?.as_ref());
    let tf_path = Path::new(testfile_path);
    let mut checksum_with_filename = String::new(); // Initialize the variable
    if let Some(testfile_name) = tf_path.file_name() {
        if let Some(testfile_name_str) = testfile_name.to_str() {
            checksum_with_filename = format!("{}  {}", calculated_checksum, testfile_name_str.to_uppercase());
        }
    } else {
        eprintln!("Invalid path or no file name found.");
    }

    Ok(checksum_with_filename == checksum_data_uppercase)
}

fn create_temp_dir() -> PathBuf {
    // Specify the absolute path for the permanent directory
    let temp_dir = Path::new("test_dir");

    // Create the directory if it doesn't exist
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    }
    temp_dir.to_path_buf()
}    

fn sha256_digest<R: io::Read>(mut reader: R) -> Result<ring::digest::Digest, std::io::Error> {
    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
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
fn check_permissions(filepath: &str, perms: u32) -> Result<bool, std::io::Error> {
    let file = fs::File::open(filepath)?;
    let metadata = file.metadata()?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    let chmod = mode_to_chmod(mode);
    Ok(chmod == perms)
}
}
