{
  description = "Steering behaviours";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        stable-rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            simple-http-server
            stable-rust
            rust-analyzer
            wasm-pack
            wasm-bindgen-cli # Needed for wasm-pack to work
            openssl
            pkgconfig
            rust-bin.beta.latest.default
          ];

          shellHook = ''
            alias gst='git status'
            alias gd='git diff'
            alias gc='git commit'
            :q() {
              exit
            }
          '';
        };
      }
    );
}

