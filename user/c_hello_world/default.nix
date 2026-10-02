{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.c_hello_world = {
        release = {
          name = "c_hello_world";
          pkg = self.lib.singleFileInitBuild "c_hello_world" "release" ./. "main.c";
        };
        debug = {
          name = "c_hello_world";
          pkg = self.lib.singleFileInitBuild "c_hello_world" "debug" ./. "main.c";
        };
      };
    };
}
