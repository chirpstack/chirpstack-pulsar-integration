{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-23.05.tar.gz") {} }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.cacert
    pkgs.rustup
    pkgs.protobuf
    pkgs.cargo-cross
    pkgs.openssl
    pkgs.cargo-deb
  ];
  DOCKER_BUILDKIT = "1";
  NIX_STORE = "/nix/store";
}
