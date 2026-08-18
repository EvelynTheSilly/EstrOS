{ inputs, ... }:
let
  helpers = import ./_helpers.nix {
    nixpkgs = inputs.nixpkgs;
    rust-overlay = inputs.rust-overlay;
  };
in
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  perSystem =
    { pkgs, ... }:
    {
      packages.rust = helpers.makeRustToolchain (pkgs.extend inputs.rust-overlay.overlays.default);
    };
}
