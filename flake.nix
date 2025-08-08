{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk' = pkgs.callPackage naersk { };
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
          ];
      in
      {
        packages.makepng = naersk'.buildPackage {
          pname = "makepng";
          src = ./.;
        };

        packages.unmakepng = naersk'.buildPackage {
          pname = "unmakepng";
          src = ./.;
        };

        packages.viewpng = naersk'.buildPackage {
          pname = "viewpng";
          src = ./.;
        };

        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            LD_LIBRARY_PATH = libPath;
          };
      }
    );
}
