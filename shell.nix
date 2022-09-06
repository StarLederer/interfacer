{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = with pkgs; [
    gcc
    rustup
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
  ];
  shellHook = ''
    export TMPDIR="/tmp"
  '';
}
