{
  description = "A command line interface for stregsystemet by fklubben AAU";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      lib = nixpkgs.lib;
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      # generates attrs like: {
      #   x86_64-linux = f nixpkgs.legacyPackages.x86_64-linux;
      #   x86_64-darwin = f nixpkgs.legacyPackages.x86_64-darwin;
      #   ...
      # }
      forAllSystems = f: lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in
    {
      # nix fmt -- ./**/*.nix
      formatter = forAllSystems (pkgs: pkgs.nixfmt);

      packages = forAllSystems (
        pkgs:
        let
          # run 'nix run nixpkgs#crate2nix -- generate` after every update to cargo.lock
          cargo-nix = pkgs.callPackage ./Cargo.nix { inherit nixpkgs pkgs; };
        in
        rec {
          default = secure-sports-cola;
          secure-sports-cola = cargo-nix.rootCrate.build.overrideAttrs {
            strictDeps = true;
            meta = {
              description = "A command line interface for stregsystemet by fklubben AAU";
              platforms = lib.platforms.linux ++ lib.platforms.darwin;
              mainProgram = "ssc";
            };
          };
        }
      );
    };
}
