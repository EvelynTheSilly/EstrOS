{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.thread_test = {
        release = {
          name = "thread_test";
          pkg = self.lib.singleFileInitBuild "thread_test" "release" ./. "main.c";
        };
        debug = {
          name = "thread_test";
          pkg = self.lib.singleFileInitBuild "thread_test" "debug" ./. "main.c";
        };
      };
    };
}