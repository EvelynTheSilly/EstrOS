{
  inputs,
  self,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  perSystem = {
    self',
    pkgs,
    ...
  }: {
    packages = {
      kernel_elf = self.lib.buildKernel {
        inherit pkgs;
        rust = self'.packages.rust;
      };
      kernel_elf_debug = self.lib.buildKernel {
        inherit pkgs;
        buildType = "debug";
        rust = self'.packages.rust;
      };
    };
  };
}
