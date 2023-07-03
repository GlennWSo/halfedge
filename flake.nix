{
  description = "A devShell that can run three-d examples";

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

        rust = pkgs.rust-bin.stable.latest.default;
        deps = with pkgs; [
          openssl
          pkg-config
          fontconfig
          cmake
        ];

        
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
         ];    

        cargo_script = pkgs.writeScriptBin "cargo" ''
          echo running cargo with add modified LD_LIBRARY_PATH
          export LD_LIBRARY_PATH=${libPath}
          export PATH=$PATH:${rust}/bin
          ${rust}/bin/cargo "$@"
        '';

        utils = with pkgs; [
          # checks video driver info
          pciutils 
          glxinfo
          nil
          gdb
          lldb
          rust-analyzer
          gitui
          cargo_script
          rust
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "rust graphics env"; 
          DERP = rust;
          buildInputs = deps ++ utils;
          shellHook = ''
            echo Hello, Dev!
          '';
        };
      }
    );
}
