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

  src = ./.;
  cargoLock.lockFile = "${finalAttrs.src}/Cargo.lock";

  meta = with lib; {
    inherit (manifest) description homepage;
    license = licenses.mit;
  };
})
