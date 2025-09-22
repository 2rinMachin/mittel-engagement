{
  description = "API for managing engagement on the Mittel blogging platform.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs@{
      self,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem =
        {
          self',
          pkgs,
          system,
          ...
        }:
        {
          packages = {
            mittel-engagement = pkgs.callPackage ./. { };
            default = self'.packages.mittel-engagement;
          };

          devShells.default = pkgs.callPackage ./shell.nix {
            inherit (self'.packages) mittel-engagement;
          };
        };
    };
}
