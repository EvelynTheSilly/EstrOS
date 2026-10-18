{
  self,
  inputs,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.net_driver = {
      release = {
        name = "net_driver";
        pkg = self.lib.multiFileInitBuild {
          name = "net_driver";
          buildType = "release";
          src = ./.;
          initName = "uart.c";
          cFiles = [ { file = "net.c"; sym = "send_process_elf"; } ];
        };
      };
      debug = {
        name = "net_driver";
        pkg = self.lib.multiFileInitBuild {
          name = "net_driver";
          buildType = "debug";
          src = ./.;
          initName = "uart.c";
          cFiles = [ { file = "net.c"; sym = "send_process_elf"; } ];
        };
      };
    };
  };
}
