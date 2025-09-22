{
  mkShell,

  sqlx-cli,
}:
mkShell {
  packages = [
    sqlx-cli
  ];

  env = {
    RUST_BACKTRACE = "1";
  };
}
