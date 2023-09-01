mod extractors;
use extractors::{extract_zip, extract_rar, extract_tar};
fn main() {
    std::process::exit(run());
}

    /*
     TODO: [ ] setup rayon to handle concurrent file processsing when passed
     more than one file
    */
    /*
     TODO: [ ] Write for loop to iter over all extensions of a file to handle
     files that are tared and then compressed ex: foo.tar.gz, foo.tar.gz
    */
    /*
     TODO: [ ] add support for decompression of:
     [ ] bz2
     [ ] tbz2
     [ ] tgz
     [ ] txz
     [ ] lzma
     [ ] gz
     [ ] z
     [ ] 7z
     [ ] arj
     [ ] cab
     [ ] arj
     [ ] cab
     [ ] chm
     [ ] deb
     [ ] dmg
     [ ] iso
     [ ] lzh
     [ ] msi
     [ ] rpm
     [ ] udf
     [ ] wim
     [ ] xar
     [ ] exe
    */
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
            // "xz" => {
            //     if let Err(err) = extract_tar(&fname) {
            //         println!("Error extracting XZ: {}", err);
            //         return 1;
            //     }
            // }
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