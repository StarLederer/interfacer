{
  description = "Interfacer";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = with fenix.packages.${system}; combine [
          default.cargo
          default.clippy
          default.rustc
          default.rustfmt
        ];
      in {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            nodejs-16_x
            nodePackages.pnpm

            rust
            rust-analyzer
          ];

          buildInputs = with pkgs; [
            # These probably can be nativeBuildInputs
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
        };
      }
    );
}
