{pkgs, ...}: {
  packages = [
    pkgs.git
    pkgs.rustup
    pkgs.openssl
    pkgs.cargo-nextest
    pkgs.taplo
    pkgs.cargo-insta
    pkgs.protobuf
  ];
}
