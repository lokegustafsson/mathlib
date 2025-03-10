{
  inputs = {
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachSystem
    [ inputs.flake-utils.lib.system.x86_64-linux ] (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        lib = pkgs.lib;

        rust-stable = pkgs.rust-bin.stable.latest.default;

        cargoNix = import ./Cargo.nix {
          inherit pkgs;
          buildRustCrateForPkgs = crate:
            pkgs.buildRustCrate.override {
              rustc = rust-stable;
              cargo = rust-stable;
              defaultCrateOverrides = crate.defaultCrateOverrides // {
                gmp-mpfr-sys = attrs: {
                  buildInputs = [ pkgs.gmp pkgs.gmp.dev ];
                };
              };
            };
        };

        mathlib = cargoNix.workspaceMembers.mathlib.build;
      in {
        formatter = pkgs.writeShellApplication {
          name = "format";
          runtimeInputs = [ rust-stable pkgs.nixfmt-classic ];
          text = ''
            set -v
            cargo fmt
            find . -name '*.nix' | grep -v Cargo.nix | xargs nixfmt'';
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = let p = pkgs;
          in [
            p.bashInteractive
            p.gmp
            p.gmp.dev
            (p.crate2nix.override { cargo = rust-stable; })
            (p.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ];

          shellHook = ''
            git rev-parse --is-inside-work-tree > /dev/null && [ -n "$CARGO_TARGET_DIR_PREFIX" ] && \
            export CARGO_TARGET_DIR="$CARGO_TARGET_DIR_PREFIX$(git rev-parse --show-toplevel)"
            exec zsh
          '';
        };

        checks.default = mathlib.override { runTests = true; };
      });
}
