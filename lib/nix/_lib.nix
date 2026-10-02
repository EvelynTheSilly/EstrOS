{ inputs, ... }:

let
  defineInit =
    {
      name,
      pkg,
    }:
    {
      inherit name pkg;
    };

  buildInit =
    {
      init,
    }:
    init.pkg;

  buildKernel =
    {
      pkgs,
      buildType ? "release",
      rust ? null,
    }:
    let
      rust' = if rust != null then rust else inputs.self.packages.${pkgs.system}.rust;
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust';
    in
    import ../../kernel {
      inherit craneLib pkgs buildType;
      rust = rust';
    };

  # Expands --release / --kernel-release / --init-release into kernel_variant
  # and init_variant. The recognised flags are collected into variant_flags and
  # everything else into variant_args, so a consumer can forward one without the
  # other: kdebug hands variant_flags to the gdb wrapper and variant_args to
  # qemu, while the wrapper keeps variant_flags to itself and hands only
  # variant_args to real gdb. Sharing this keeps the booted image and the loaded
  # symbols from ever disagreeing.
  variantArgs =
    {
      defaultKernel ? "debug",
      defaultInit ? "debug",
    }:
    ''
      kernel_variant=${defaultKernel}
      init_variant=${defaultInit}
      variant_flags=()
      variant_args=()
      for arg in "$@"; do
        case "$arg" in
          --release)
            kernel_variant=release
            init_variant=release
            variant_flags+=("$arg")
            ;;
          --kernel-release)
            kernel_variant=release
            variant_flags+=("$arg")
            ;;
          --init-release)
            init_variant=release
            variant_flags+=("$arg")
            ;;
          *)
            variant_args+=("$arg")
            ;;
        esac
      done
    '';

in
{
  flake =
    { ... }:
    {
      lib = {
        inherit defineInit buildInit buildKernel variantArgs;
      };
    };
}
