{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  darwin,
  stdenv,
}:

rustPlatform.buildRustPackage {
  pname = "net-reduce";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoHash = "sha256-U4RAL2/4I/ovUA3/PUuxCHpLZw27fNuqlW5N/tJ0iOg=";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs =
    [
      openssl
    ]
    ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.SystemConfiguration
    ];

  meta = with lib; {
    description = "Simple tool for reducing (removing more specifics) CIDR/IP addresses from standard input";
    homepage = "https://github.com/czerwonk/net-reduce";
    license = licenses.mit;
  };
}
