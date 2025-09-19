{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs; [
    openssl
    pkg-config
    sqlx-cli
  ];

  env = {
    RUST_BACKTRACE = "1";
  };
}
