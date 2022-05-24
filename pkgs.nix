let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/f233fdc4ff6ba2ffeb1e3e3cd6d63bb1297d6996.tar.gz");
  #"https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import (builtins.fetchTarball
    "https://github.com/nixos/nixpkgs/archive/21.11.tar.gz");
in nixpkgs {
  overlays = [
    moz_overlay
    (self: super:
      let
        channel = self.rustChannelOf {
          date = "2022-04-17";
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
