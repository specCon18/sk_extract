{
  description = "Spec's NixOS-config";
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
          xz  # Add xz (liblzma) to the build inputs
        ];

        # Add a shellHook to modify PKG_CONFIG_PATH
        shellHook = ''
          export PKG_CONFIG_PATH="${pkgs.xz}/lib/pkgconfig:$PKG_CONFIG_PATH"
        '';
      };
    };
}
