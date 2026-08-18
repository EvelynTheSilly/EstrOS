{ nixpkgs, rust-overlay }:

let
  makeRustToolchain =
    pkgs:
    let
      toml = builtins.fromTOML (builtins.readFile ../../rust-toolchain.toml);
      tc = toml.toolchain;
    in
    pkgs.rust-bin.fromRustupToolchain {
      channel = tc.channel;
      components = tc.components or [ ];
      targets = tc.targets or [ ];
    };

  makeEstrosCC = pkgs: import ../c/default.nix { inherit pkgs; };

  buildInit =
    {
      system ? builtins.currentSystem,
      src,
    }:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
        config.allowUnsupportedSystem = true;
      };
      cc = makeEstrosCC pkgs;
      cross = pkgs.pkgsCross.aarch64-embedded;
      rust = makeRustToolchain pkgs;
      buildEnv = {
        inherit pkgs cross rust;
        cc = cc.packages.aarch64-estros-binutils;
        opt.level = "debug";
      };
    in
    import src buildEnv;

in
{
  inherit makeRustToolchain makeEstrosCC buildInit;
}
