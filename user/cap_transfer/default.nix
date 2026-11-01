{
  self,
  inputs,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.cap_transfer = {
      release = {
        name = "cap_transfer";
        pkg = self.lib.multiFileInitBuild {
          name = "net_driver";
          buildType = "release";
          src = ./.;
          initName = "spawner.c";
          cFiles = [ { file = "uart.c"; sym = "uart_process"; } ];
        };
      };
      debug = {
        name = "cap_transfer";
        pkg = self.lib.multiFileInitBuild {
          name = "net_driver";
          buildType = "debug";
          src = ./.;
          initName = "spawner.c";
          cFiles = [ { file = "uart.c"; sym = "uart_process"; } ];
        };
      };
    };
  };
}
