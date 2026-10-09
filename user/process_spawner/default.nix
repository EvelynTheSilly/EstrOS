{ self, inputs, ... }:
{
  imports = [ inputs.flake-parts.flakeModules.nixpkgs ];

  flake =
    { ... }:
    {
      estros.inits.process_spawner = {
        release = {
          name = "process_spawner";
          pkg = self.lib.multiFileInitBuild {
            name = "process_spawner";
            buildType = "release";
            src = ./.;
            initName = "main.c";
            cFiles = [ { file = "second.c"; sym = "second_process_elf"; } ];
          };
        };
        debug = {
          name = "process_spawner";
          pkg = self.lib.multiFileInitBuild {
            name = "process_spawner";
            buildType = "debug";
            src = ./.;
            initName = "main.c";
            cFiles = [ { file = "second.c"; sym = "second_process_elf"; } ];
          };
        };
      };
    };
}
