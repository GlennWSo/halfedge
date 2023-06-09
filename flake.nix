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
          pkgconfig
          fontconfig
          cmake
        ];

        utils = with pkgs; [
          # checks video driver info
          pciutils 
          glxinfo
          nil
          gdb
          lldb
          rust-analyzer
          gitui
        ];
        
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
         ];    
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "rust graphics env"; 
          buildInputs = [rust] ++ deps ++ utils;
          LD_LIBRARY_PATH=libPath;
          shellHook = ''
            echo Hello, Dev!
          '';
        };
      }
    );
}
