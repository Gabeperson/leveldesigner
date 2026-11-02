{
  description = "Rust + OpenGL";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildInputs = with pkgs; [
          libGL
          libX11
          libXi
          libxkbcommon
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            buildInputs
            ++ [
              "/run/opengl-driver/lib"
            ]
          );

          shellHook = ''
            echo "Flake initialized"
            exec fish
          '';
        };
      }
    );
}
