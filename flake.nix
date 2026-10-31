{
  description = "fmc flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    bevy-flake = {
      url = "github:swagtop/bevy-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      bevy-flake,
      rust-overlay,
      ...
    }:
    bevy-flake.lib.mkFlake {
      perSystem =
        {
          pkgs,
          system,
          packages,
          formatter,
          ...
        }:
        {
          inherit packages formatter;

          devShells.default = pkgs.mkShell {
            name = "Fmc";
            packages = [
              packages.rust-toolchain.develop
              pkgs.wgsl-analyzer
            ];
          };
        };

      config =
        {
          pkgs,
          system,
          ...
        }:
        {
          src = builtins.path {
            name = "src";
            path = ./.;

            # Ignore files that aren't needed in compilation of Bevy project.
            filter =
              path: type:
              !(builtins.elem (baseNameOf path) [
                "flake.lock"
                "flake.nix"
              ]);
          };

          rustToolchain =
            targets:
            let
              channel = "nightly";
            in
            pkgs.rust-bin.${channel}.latest.default.override {
              inherit targets;
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            };

          withPkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
            config = {
              allowUnfree = true;
              microsoftVisualStudioLicenseAccepted = true;
            };
          };
        };
    };
}
