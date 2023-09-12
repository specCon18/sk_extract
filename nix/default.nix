{ pkgs ? import <nixpkgs> { } }:

let
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgs.lib.cleanSource ../.;
  buildInputs = [ pkgs.xz pkgs.unrar ];
  nativeBuildInputs = [ pkgs.xz pkgs.pkg-config ];
}