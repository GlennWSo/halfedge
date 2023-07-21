{
  description = "A devShell that can run three-d examples";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils}:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default;
        
        graphicLibs = with pkgs; lib.makeLibraryPath [
          libGL
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
         ];    

        cargo_script = pkgs.writeScriptBin "car" ''
          echo running cargo with add modified LD_LIBRARY_PATH
          export LD_LIBRARY_PATH=${graphicLibs}
          export PATH=$PATH:${rust}/bin
          ${rust}/bin/cargo "$@"
        '';

        buildDeps = with pkgs; [
          openssl
          pkg-config
          cargo_script
          rust
        ];

        utils = with pkgs; [
          #  video driver info
          pciutils 
          glxinfo
          nil
          gdb
          lldb
          rust-analyzer
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "rust graphics env"; 
          DERP = rust;
          buildInputs = buildDeps ++ utils;
          shellHook = ''
            echo Entering rust env!
          '';
        };
      }
    );
}
