{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.send = {
        release = {
          name = "send";
          pkg = self.lib.singleFileInitBuild "send" "release" ./. "send.c";
        };
        debug = {
          name = "send";
          pkg = self.lib.singleFileInitBuild "send" "debug" ./. "send.c";
        };
      };
      estros.inits.receive = {
        release = {
          name = "receive";
          pkg = self.lib.singleFileInitBuild "receive" "release" ./. "receive.c";
        };
        debug = {
          name = "receive";
          pkg = self.lib.singleFileInitBuild "receive" "debug" ./. "receive.c";
        };
      };
    };
}