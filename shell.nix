{ pkgs ? import <nixpkgs> {
  overlays = [ (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
} }:

let
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
    # targets = [ "arm-unknown-linux-gnueabihf" ];
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rust
    cargo
    rustfmt
    rust-analyzer

    nodejs-16_x
    nodePackages.pnpm

    pkgconfig
    openssl
    glib
    cairo
    pango
    atk
    gdk-pixbuf
    libsoup
    gtk3
    webkitgtk
    librsvg
    patchelf
    dbus
    SDL2
  ];

  RUST_BACKTRACE = 1;

  shellHook = ''
    export TMPDIR="/tmp"
  '';
}
