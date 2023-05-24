{
  description = "A devShell that can run three-d examples";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk,... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.default;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust;
          rustc = rust;
        };

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
      rec {
        devShells.default = mkShell {
          name = "rust graphics env"; 
          buildInputs = [rust] ++ deps ++ utils;
          LD_LIBRARY_PATH=libPath;
          shellHook = ''
            echo Hello, Dev!
          '';
        };

        packages.triangle-example = naersk-lib.buildPackage {
            name = "three-d.triangle-example";
            pname = "triangle";
            src = ./.;
            buildInputs = deps;
            cargoBuildOptions = defaultOptions: defaultOptions ++ ["--examples"];
          };

        packages.fireworks-example = naersk-lib.buildPackage {
            name = "three-d.fireworks-example";
            pname = "fireworks";
            src = ./.;
            buildInputs = deps;
            cargoBuildOptions = defaultOptions: defaultOptions ++ ["--examples"];
          };

        packages.default = packages.triangle-example;

      }
    );
}
