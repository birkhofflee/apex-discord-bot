{
  description = "apex-discord-bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        commonNativeBuildInputs = with pkgs; [
          pkg-config
        ];

        commonBuildInputs = with pkgs;
          lib.optionals (!stdenv.isDarwin) [ openssl ]
          ++ lib.optionals stdenv.isDarwin [ libiconv ];
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "apex-discord-bot";
          version = "0.1.0";
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs ++ (with pkgs; [
            rustc
            cargo
            rust-analyzer
            just
          ]);
        };
      }
    );
}
