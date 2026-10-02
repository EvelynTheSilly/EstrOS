{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    import-tree.url = "github:denful/import-tree";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    imports = [
      (inputs.import-tree ./lib/nix)
      (import ./lib/nix/_lib.nix)
      (import ./lib/nix/qemu/_disk_image.nix)
      (import ./lib/nix/qemu/_runners.nix)
      (import ./user/_lib.nix)
      (import ./lib/c/default.nix)
      (inputs.import-tree ./user)
    ];
  };
}
