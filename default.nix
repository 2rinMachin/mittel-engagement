{
  lib,
  rustPlatform,
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = manifest.name;
  inherit (manifest) version;

  src = lib.cleanSourceWith {
    src = ./.;
    filter = name: type: (lib.cleanSourceFilter name type) || (baseNameOf name == ".sqlx");
  };
  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

  env = {
    SQLX_OFFLINE = "true";
  };

  meta = with lib; {
    inherit (manifest) description homepage;
    license = licenses.mit;
  };
})
