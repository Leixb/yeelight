{
  description = "Rust CLI and API bindings for yeelight WiFi Light Inter-Operation";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit-hooks.url = "github:hercules-ci/pre-commit-hooks.nix/flakeModule";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-parts, pre-commit-hooks, ... }@inputs:

    flake-parts.lib.mkFlake { inherit inputs; } {

      imports = [
        pre-commit-hooks.flakeModule
      ];

      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = { config, self', inputs', pkgs, system, ... }: {

        pre-commit.settings.hooks = {
          nixpkgs-fmt.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
        };

        devShells.default = pkgs.mkShell {
          name = "cargo";
          buildInputs = with pkgs; [
            cargo
            cargo-edit
            cargo-release
            cargo-outdated
            clippy
            rust-analyzer
            rustc
            rustfmt
            lldb
          ];

          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
        };

        packages.default = pkgs.callPackage ./default.nix { };
      };
    };
}
