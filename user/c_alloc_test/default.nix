{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.c_alloc_test = {
        release = {
          name = "c_alloc_test";
          pkg = self.lib.singleFileInitBuild "c_alloc_test" "release" ./. "main.c";
        };
        debug = {
          name = "c_alloc_test";
          pkg = self.lib.singleFileInitBuild "c_alloc_test" "debug" ./. "main.c";
        };
      };
    };
}