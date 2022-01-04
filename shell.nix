let
pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz") {};
in pkgs.mkShell {
    name = "steering";


    # Required for rust-analyzer?
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    nativeBuildInputs = [
      pkgs.rustc
      pkgs.cargo
    ];

    buildInputs = [
      pkgs.yarn               # coc.vim
      pkgs.nodejs-14_x        # coc.vim
      pkgs.rustfmt
      pkgs.simple-http-server # For development
      # pkgs.wasm-pack          # A utility that builds rust-generated WebAssembly
    ];
    shellHook = ''
      alias gst='git status'
      alias gd='git diff'
      alias gc='git commit'
      :q() {
        exit
      }
    '';
  }
