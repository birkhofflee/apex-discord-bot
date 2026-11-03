{
  description = "apex-discord-bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          inherit pkgs;

          commonNativeBuildInputs = with pkgs; [
            pkg-config
          ];

          commonBuildInputs = with pkgs;
            lib.optionals (!stdenv.hostPlatform.isDarwin) [ openssl ]
            ++ lib.optionals stdenv.hostPlatform.isDarwin [ libiconv ];
        });
    in
    {
      packages = forAllSystems (system:
        with perSystem.${system}; {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "apex-discord-bot";
            version = "0.1.0";
            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = commonBuildInputs;
          };
        });

      devShells = forAllSystems (system:
        with perSystem.${system}; {
          default = pkgs.mkShell {
            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = commonBuildInputs ++ (with pkgs; [
              rustc
              cargo
              rust-analyzer
              just
            ]);
          };
        });
    };
}
