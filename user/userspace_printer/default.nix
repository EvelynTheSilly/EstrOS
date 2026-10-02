{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.userspace_printer = {
        release = {
          name = "userspace_printer";
          pkg = self.lib.singleFileInitBuild "userspace_printer" "release" ./. "main.c";
        };
        debug = {
          name = "userspace_printer";
          pkg = self.lib.singleFileInitBuild "userspace_printer" "debug" ./. "main.c";
        };
      };
    };
}