#!/usr/bin/env bash
: "${ESTROS_RA:?ESTROS_RA not set - enter the nix dev shell before launching zed}"
# RA probes the target by spawning `rustc`; the dev-shell PATH rustc is the
# upstream toolchain, which doesn't know aarch64-estros, so pin the fork's.
RUSTC="$(dirname "$ESTROS_RA")/rustc"
export RUSTC
exec "$ESTROS_RA" "$@"
