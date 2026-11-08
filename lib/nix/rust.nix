{inputs, ...}: let
  makeRustToolchain = pkgs: let
    toml = builtins.fromTOML (builtins.readFile ../../rust-toolchain.toml);
    tc = toml.toolchain;
  in
    pkgs.rust-bin.fromRustupToolchain {
      channel = tc.channel;
      components = tc.components or [];
      targets = tc.targets or [];
    };
in {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  perSystem = {pkgs, ...}: {
    packages.rust = makeRustToolchain (pkgs.extend inputs.rust-overlay.overlays.default);
  };
}
