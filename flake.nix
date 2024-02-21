{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
   {
    devShells.${system}.default = pkgs.mkShell {
      shellHook = "echo Ready to develop Shuftle!";
      packages = [
        pkgs.cargo
        pkgs.rustc
        pkgs.rust-analyzer
        pkgs.cargo-watch
        pkgs.clippy
        pkgs.rustfmt
      ];
    };
  };
}
