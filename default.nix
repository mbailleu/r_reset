{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  name = "r-reset";
  src = ./.;
  cargoSha256 = "1w5vanzrxbbmf6a2k7f9scfqkj7g5xp9a0rq5zbgijpqsdr3lqfq";
}
