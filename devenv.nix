{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = [pkgs.git pkgs.rustup pkgs.openssl pkgs.cargo-nextest pkgs.taplo pkgs.protobuf];
}
