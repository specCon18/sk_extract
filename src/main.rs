/*
TODO_3: setup rayon to handle parallel file processsing when passed
more than one file and square up concurrency model for files with
more than one extension.
*/
/*
TODO_1: define supported extensions as structs and write an extensions enum
*/
mod extractors;
// mod extensions;
// use extenstions::{
    // File,
    // Extensions
// }
use std::{path::Path, fs};
use extractors::{
    extract_zip,
    extract_rar,
    extract_tar,
    extract_xz,
    extract_bz2,
    // extract_tbz2,
    // extract_tgz,
    // extract_txz,
    // extract_lzma,
    extract_gz,
    // extract_z,
    // extract_7z,
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
fn main() {
    std::process::exit(run());
}
fn run() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    if let Some(extension) = fname.extension().and_then(|s| s.to_str()) {
        match extension {
            "zip" => {
                if let Err(err) = extract_zip(&fname) {
                    println!("Error extracting ZIP: {}", err);
                    return 1;
                }
            }
            "rar" => {
                if let Err(err) = extract_rar(&fname) {
                    println!("Error extracting RAR: {}", err);
                    return 1;
                }
            }
            "tar" => {
                if let Err(err) = extract_tar(&fname) {
                    println!("Error extracting TAR: {}", err);
                    return 1;
                }
            }
            "xz" => {
                let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                if let Err(err) = fs::create_dir_all(&output_directory) {
                    println!("Error creating output directory: {}", err);
                    return 1;
                }
                if let Err(err) = extract_xz(&fname, &output_directory) {
                    println!("Error extracting XZ: {}", err);
                    return 1;
                }
            }
            "gz" => {
                let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                if let Err(err) = extract_gz(&fname, &output_directory) {
                    println!("Error extracting GZ: {}", err);
                    return 1;
                }
            }
            "bz2" => {
                let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                if let Err(err) = extract_bz2(&fname, &output_directory) {
                    println!("Error extracting BZ2: {}", err);
                    return 1;
                }
            }
            /*
            "tbz2" => {
                if let Err(err) = extract_tbz2(&fname) {
                    println!("Error extracting TBZ2: {}", err);
                    return 1;
                }
            }
            "txz" => {
                if let Err(err) = extract_txz(&fname) {
                    println!("Error extracting TXZ: {}", err);
                    return 1;
                }
            }
            "tgz" => {
                if let Err(err) = extract_tgz(&fname) {
                    println!("Error extracting TGZ: {}", err);
                    return 1;
                }
            }
            "lzma" => {
                if let Err(err) = extract_lzma(&fname) {
                    println!("Error extracting LZMA: {}", err);
                    return 1;
                }
            }
            "z" => {
                if let Err(err) = extract_z(&fname) {
                    println!("Error extracting Z: {}", err);
                    return 1;
                }
            }
            "7z" => {
                if let Err(err) = extract_7z(&fname) {
                    println!("Error extracting 7Z: {}", err);
                    return 1;
                }
            }
            "arj" => {
                if let Err(err) = extract_arj(&fname) {
                    println!("Error extracting ARJ: {}", err);
                    return 1;
                }
            }
            "cab" => {
                if let Err(err) = extract_cab(&fname) {
                    println!("Error extracting CAB: {}", err);
                    return 1;
                }
            }
            "chm" => {
                if let Err(err) = extract_chm(&fname) {
                    println!("Error extracting CHM: {}", err);
                    return 1;
                }
            }
            "deb" => {
                if let Err(err) = extract_deb(&fname) {
                    println!("Error extracting DEB: {}", err);
                    return 1;
                }
            }
            "dmg" => {
                if let Err(err) = extract_dmg(&fname) {
                    println!("Error extracting DMG: {}", err);
                    return 1;
                }
            }
            "iso" => {
                if let Err(err) = extract_iso(&fname) {
                    println!("Error extracting ISO: {}", err);
                    return 1;
                }
            }
            "lzh" => {
                if let Err(err) = extract_lzh(&fname) {
                    println!("Error extracting LZH: {}", err);
                    return 1;
                }
            }
            "msi" => {
                if let Err(err) = extract_msi(&fname) {
                    println!("Error extracting MSI: {}", err);
                    return 1;
                }
            }
            "rpm" => {
                if let Err(err) = extract_rpm(&fname) {
                    println!("Error extracting RPM: {}", err);
                    return 1;
                }
            }
            "udf" => {
                if let Err(err) = extract_udf(&fname) {
                    println!("Error extracting UDF: {}", err);
                    return 1;
                }
            }
            "wim" => {
                if let Err(err) = extract_wim(&fname) {
                    println!("Error extracting WIM: {}", err);
                    return 1;
                }
            }
            "xar" => {
                if let Err(err) = extract_udf(&fname) {
                    println!("Error extracting XAR: {}", err);
                    return 1;
                }
            }
            "exe" => {
                if let Err(err) = extract_exe(&fname) {
                    println!("Error extracting EXE: {}", err);
                    return 1;
                }
            }
            */
            _ => {
                println!("Unsupported file extension: {}", extension);
                return 1;
            }
        }
        return 0;
    }

    println!("Unknown file format");
    return 1;
}