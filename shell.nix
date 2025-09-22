{
  mkShell,

  mittel-engagement,
  awscli2,
  sqlx-cli,
}:
mkShell {
  inputsFrom = [
    mittel-engagement
  ];

  packages = [
    awscli2
    sqlx-cli
  ];

  env = {
    RUST_BACKTRACE = "1";
  };
}
