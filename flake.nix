{
  description = "Development shell for a generic Rust project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs: {
    devShells = {
      x86_64-linux = {
        default =
          let
            pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
          in
          pkgs.mkShell {
            buildInputs = [
              pkgs.rustup
            ];
          };
      };
    };
  };
}
