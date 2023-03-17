with (import ./pkgs.nix {});
with llvmPackages;
mkShell {
  nativeBuildInputs = [ git clang ];
  buildInputs = [ pkgsStatic.openssl.dev pkg-config rust-nightly dprint cargo-sort ];
  RUST_SRC_PATH = "${rust-nightly}/lib/rustlib/src/rust/src";
  LIBCLANG_PATH = "${clang-unwrapped.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  ROCKSDB_STATIC = "${rocksdb}/lib";
  OPENSSL_STATIC = "true";
  LIBZ_SYS_STATIC = "1";
}
