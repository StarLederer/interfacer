{
  description = "Bevy";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    rust = with fenix.packages.${system}; combine [
      default.cargo
      default.clippy
      default.rustc
      default.rustfmt
    ];
  in {
    devShell.${system} = pkgs.mkShell rec {
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

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        shellHook = ''
          cd bevy
        '';
      };
  };
}
