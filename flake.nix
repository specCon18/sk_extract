{
  description = "SK Extract";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }@inputs:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./nix/default.nix { };
      });
      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./nix/devshell.nix { };
      });
    };
}