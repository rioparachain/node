with (import ./pkgs.nix);
with llvmPackages;
mkShell {
  nativeBuildInputs = [ clang ];
  buildInputs = [ git nixfmt crate2nix rust-nightly pkg-config openssl cmake ];
  shellHook = ''
    set -e
    export LIBCLANG_PATH="${clang-unwrapped.lib}/lib";
    export PROTOC="${protobuf}/bin/protoc";
    export ROCKSDB_LIB_DIR="${rocksdb}/lib";
  '';
}
