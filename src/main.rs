use std::{path::Path, fs};
use rayon::prelude::*;
use sk_extract::extractors::{
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
    extract_deb,
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
        println!("Usage: {} <filename1> [<filename2> ...]", args[0]);
        return 1;
    }

    // Collect all input file paths from command line arguments
    let file_paths: Vec<_> = args.iter().skip(1).collect();

    // Use Rayon to parallelize the extraction process
    let result = file_paths.par_iter().map(|file_path| {
        let fname = std::path::Path::new(file_path);
        if let Some(extension) = fname.extension().and_then(|s| s.to_str()) {
            match extension {
                "zip" => {
                    //Extract Files from a ZIP archive
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_zip(&fname, &output_directory) {
                        println!("Error extracting ZIP: {}", err);
                        return 1;
                    }
                }
                "rar" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_rar(&fname, &output_directory) {
                        println!("Error extracting RAR: {}", err);
                        return 1;
                    }
                }
                "tar" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_tar(&fname,&output_directory) {
                        println!("Error extracting TAR: {}", err);
                        return 1;
                    }
                }
                "xz" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if fname.to_str().unwrap().ends_with(".tar.xz") {
                        if let Err(err) = extract_txz(&fname,&output_directory) {
                            println!("Error extracting TXZ: {}", err);
                            return 1;
                        }
                    } else {
                        if let Err(err) = fs::create_dir_all(&output_directory) {
                            println!("Error creating output directory: {}", err);
                            return 1;
                        }
                        if let Err(err) = extract_lzma(&fname, &output_directory) {
                            println!("Error extracting XZ: {}", err);
                            return 1;
                        }
                    }
                }
                "gz" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if fname.to_str().unwrap().ends_with(".tar.gz") {
                        if let Err(err) = extract_tgz(&fname,&output_directory) {
                            println!("Error extracting TGZ: {}", err);
                            return 1;
                        }
                    } else {
                        if let Err(err) = extract_gz(&fname, &output_directory) {
                            println!("Error extracting GZ: {}", err);
                            return 1;
                        }
                    }
                }
                "bz2" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if fname.to_str().unwrap().ends_with(".tar.bz2") {
                        if let Err(err) = extract_tbz2(&fname,&output_directory) {
                            println!("Error extracting TBZ2: {}", err);
                            return 1;
                        }
                    } else {
                        if let Err(err) = extract_bz2(&fname, &output_directory) {
                            println!("Error extracting BZ2: {}", err);
                            return 1;
                        }
                    }
                }
                "lzma" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_lzma(&fname, &output_directory) {
                        println!("Error extracting LZMA: {}", err);
                        return 1;
                    }
                }
                "7z" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_7z(&fname, &output_directory) {
                        println!("Error extracting 7Z: {}", err);
                        return 1;
                    }
                }
                "tbz2" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_tbz2(&fname,&output_directory) {
                        println!("Error extracting TBZ2: {}", err);
                        return 1;
                    }
                }
                "txz" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_txz(&fname,&output_directory) {
                        println!("Error extracting TXZ: {}", err);
                        return 1;
                    }
                }
                "tgz" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_tgz(&fname,&output_directory) {
                        println!("Error extracting TGZ: {}", err);
                        return 1;
                    }
                }
                "deb" => {
                    let output_directory = Path::new("output_directory"); // Change this to your desired output directory
                    if let Err(err) = fs::create_dir_all(&output_directory) {
                        println!("Error creating output directory: {}", err);
                        return 1;
                    }
                    if let Err(err) = extract_deb(&fname) {
                        println!("Error extracting DEB: {}", err);
                        return 1;
                    }
                }
                /*

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
        } else {
            println!("Unknown file format: {:?}", file_path);
            return 1;
        }
        0
    }).reduce(|| 0, |a, b| a + b);

    if result == 0 {
        0
    } else {
        1
    }
}
