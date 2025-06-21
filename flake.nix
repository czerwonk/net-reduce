{
  description = "net-reduce - Simple tool for reducing CIDR/IP addresses";

  outputs =
    { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      pkgsForSystem =
        system:
        (import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        });
    in
    {
      overlays.default =
        _final: prev:
        let
          inherit (prev) rustPlatform callPackage lib;
        in
        {
          net-reduce = callPackage ./package.nix { inherit rustPlatform lib; };
        };

      packages = forAllSystems (system: rec {
        inherit (pkgsForSystem system) net-reduce;
        default = net-reduce;
      });
    };
}
