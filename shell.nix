{
  mkShell,

  mittel-engagement,
  sqlx-cli,
}:
mkShell {
  inputsFrom = [
    mittel-engagement
  ];

  packages = [
    sqlx-cli
  ];

  env = {
    RUST_BACKTRACE = "1";
  };
}
