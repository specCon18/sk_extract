{
  description = "A tool for solving my inability to remember the right args to extract all the damnd formats";

  inputs={
    nixpkgs.url = "nixpkgs/23.05";
  };
  
  outputs = {self, nixpkgs}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

  in {
    defaultPackage.${system} = with pkgs; stdenv.mkDerivation {
      name = "extract_rs";
      src = self;

      buildInputs = [
        openssl
        pkgconfig
        rustc
        cargo
        cargo-watch
        just
      ];

      shellHook = ''
        export OPENSSL_DIR=${openssl.dev}
        export OPENSSL_LIB_DIR=${openssl.out}/lib
        export OPENSSL_INCLUDE_DIR=${openssl.dev}/include
      '';
    };
  };
}
