# convenience functions for building userspace stuff
{inputs,self, ...}:
let 
  pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
  singleFileInitBuild = 
      name:
      buildType:
      src:
      initName:
      let
        optFlag = if buildType == "debug" then "-O0" else "-O2";
      in
      pkgs.stdenv.mkDerivation {
        inherit name;
        src = src;
        nativeBuildInputs = [
          self.packages.x86_64-linux.aarch64-estros-binutils
        ];
        buildPhase = ''
          aarch64-estros-gcc ${optFlag} ${src}/${initName} -o init.elf
        '';
        installPhase = ''
          mkdir $out
          cp init.elf $out
        '';
      };
  cHeaderFromBin =
    name:
    file:
    pkgs.stdenv.mkDerivation {
      inherit name;
      src = file;
      dontUnpack = true;
      nativeBuildInputs = [
        pkgs.xxd
      ];
      buildPhase =''
        xxd -i -n ${name} ${file} > __output_file
      '';
      installPhase = ''
        cp __output_file $out
      '';
    };
  multiFileInitBuild =
    {
      name,
      buildType,
      src,
      initName,
      cFiles ? [ ],
      binFiles ? [ ],
    }:
    let
      optFlag = if buildType == "debug" then "-O0" else "-O2";

      mkElf =
        file:
        pkgs.stdenv.mkDerivation {
          name = "${name}-${file}.elf";
          inherit src;
          nativeBuildInputs = [
            self.packages.x86_64-linux.aarch64-estros-binutils
          ];
          buildPhase = ''
            aarch64-estros-gcc ${optFlag} ${file} -o __output_file
          '';
          installPhase = ''
            cp __output_file $out
          '';
        };

      headers =
        map (cf: {
          headerName = "${cf.sym}.h";
          header = cHeaderFromBin cf.sym (mkElf cf.file);
        }) cFiles
        ++ map (bf: {
          headerName = "${bf.sym}.h";
          header = cHeaderFromBin bf.sym "${src}/${bf.file}";
        }) binFiles;
    in
    pkgs.stdenv.mkDerivation {
      inherit name src;
      nativeBuildInputs = [
        self.packages.x86_64-linux.aarch64-estros-binutils
      ];
      buildPhase = ''
        ${pkgs.lib.concatMapStringsSep "\n" (h: "cp ${h.header} ${h.headerName}") headers}
        aarch64-estros-gcc ${optFlag} ${initName} -o init.elf
      '';
      installPhase = ''
        mkdir $out
        cp init.elf $out
      '';
    };
in
{
  flake =
      { ... }:
      {
        lib = {
          inherit singleFileInitBuild cHeaderFromBin multiFileInitBuild;
        };
      };
}
