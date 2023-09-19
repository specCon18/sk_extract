{ pkgs ? import <nixpkgs> { }, lib }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "sk_extract";
  version = "0.1.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgs.lib.cleanSource ../.;
  buildInputs = [ pkgs.xz pkgs.unrar ];
  nativeBuildInputs = [ pkgs.xz pkgs.pkg-config ];
#  doCheck = false;
}