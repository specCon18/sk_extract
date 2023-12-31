 # TLDR
Interface for developers and users to interact with many common archive formats.

# Installation
### install with Cargo:
```shell
  cargo build --release
```
### Install with nix:
add the flake
call the module

### Install with windows
- Install git
- Install vcpkg
```powershell
    git clone https://github.com/microsoft/vcpkg.git
    .\bootstrap-vcpkg.bat
```
- Install liblzma via vcpkg
```powershell
    vcpkg install liblzma:x64-windows-static-md
```
- Build using:
```powershell
    cargo build --release
```
# Examples
to use the cli run sk_extract [ARCHIVES]

to use the library first run:
```shell
  cargo add sk_extract
```
then import the extractors you need and call the extractor fn with parameters for `input_path` and `output_path`:
```rust
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
    extract_7z
};

            match extension {
                "zip" => {
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
```
# Credits
Thanks to all of the wonderful library devs listed below that have helped the early bootstrapping of this project:
- unrar.rs
- zip
- bzip2
- flate2
- rayon
- rust-lzma
- sevenz-rust
- tar
- serial_test
Thanks to the dood sky who helped me with the nixery!

# LICENSE:
This software is licensed under the MIT License except for any code that directly interacts with unrar which falls under [it's gross license](./rar_license.txt)
### TODO:
- add support for the remaining filetypes
