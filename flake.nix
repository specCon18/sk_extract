{
  description = "ExtractRS for all you decompression needs";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.rust = pkgs.mkShell{
        buildInputs = with pkgs; [
          cargo
          cargo-watch
          rustc
          just
          rustup
          clippy
          lolcat
          rust-analyzer
          xz
          pkg-config
          mdbook
        ];
      };
    };
}
