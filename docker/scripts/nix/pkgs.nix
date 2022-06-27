# We create a nix expression for the rust build environment, where fixed variables are used.

{ rustDate ? "2021-11-01"
, nixpkgs ? import (builtins.fetchTarball "https://github.com/nixos/nixpkgs/archive/21.11.tar.gz")
, moz_overlay ? import (builtins.fetchTarball
  "https://github.com/mozilla/nixpkgs-mozilla/archive/f233fdc4ff6ba2ffeb1e3e3cd6d63bb1297d6996.tar.gz")
}:
let
in nixpkgs {
  overlays = [
    moz_overlay
    (self: super:
      let
        channel = self.rustChannelOf {
          date = rustDate;
          channel = "nightly";
        };
        rust-nightly = channel.rust.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rustfmt-preview" ];
        };
      in {
        rustc = rust-nightly;
        cargo = rust-nightly;
        rust-nightly = rust-nightly;
      })
  ];
}
