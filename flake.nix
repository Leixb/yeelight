{
  description = "Cargo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:

    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          name = "cargo";
          buildInputs = with pkgs; [
            cargo
            cargo-edit
            cargo-release
            rust-analyzer
            rustc
            rustfmt
          ];
        };

        packages.default = pkgs.callPackage ./default.nix { };
      }
    );
}
