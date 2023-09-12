{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  # Additional tooling
  buildInputs = with pkgs; [
    cargo
    cargo-watch
    rustc
    just
    rustup
    clippy
    rust-analyzer
    xz
    pkg-config
  ];
}