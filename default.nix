{ rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "nix_sysconf";
  version = "0.0.1";

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
