{
  self,
  inputs,
  ...
}: {
  imports = [inputs.flake-parts.flakeModules.nixpkgs];

  flake = {...}: {
    estros.inits.send_recieve = {
      release = {
        name = "send_recieve";
        pkg = self.lib.multiFileInitBuild {
          name = "send_recieve";
          buildType = "release";
          src = ./.;
          initName = "receive.c";
          cFiles = [ { file = "send.c"; sym = "send_process_elf"; } ];
        };
      };
      debug = {
        name = "send_recieve";
        pkg = self.lib.multiFileInitBuild {
          name = "send_recieve";
          buildType = "debug";
          src = ./.;
          initName = "receive.c";
          cFiles = [ { file = "send.c"; sym = "send_process_elf"; } ];
        };
      };
    };
  };
}
