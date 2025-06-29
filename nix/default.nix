{
  unrar,
  xz,
  pkg-config,
  rustPlatform,
  lib,
}:

rustPlatform.buildRustPackage {
  pname = "sk_extract";
  version = "0.1.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = lib.cleanSource ../.;
  buildInputs = [
    xz
    unrar
  ];
  nativeBuildInputs = [
    xz
    pkg-config
  ];
  doCheck = false;
}
