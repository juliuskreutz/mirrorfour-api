{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devenv.url = "github:cachix/devenv";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      rust-overlay,
      devenv,
      flake-utils,
      ...
    }:
    {
      nixosModules = rec {
        default =
          {
            config,
            lib,
            pkgs,
            ...
          }:
          let
            cfg = config.services.mirrorfour-api;
          in
          {
            options.services.mirrorfour-api = {
              enable = lib.mkEnableOption "mirrourfour-api";

              database = lib.mkOption {
                type = lib.types.submodule {
                  options = {
                    name = lib.mkOption {
                      type = lib.types.str;
                      default = "mirrorfour";
                    };

                    user = lib.mkOption {
                      type = lib.types.str;
                      default = "postgres";
                    };

                    password = lib.mkOption {
                      type = lib.types.str;
                      default = "postgres";
                    };
                  };
                };
                default = { };
              };

              sessionKey = lib.mkOption {
                type = lib.types.str;
                default = null;
              };
            };

            config = lib.mkIf cfg.enable {
              assertions = [
                {
                  assertion = config.services.postgresql.enable;
                  message = "services.postgresql has to be enabled";
                }
              ];

              environment.systemPackages = [
                self.packages.${pkgs.system}.mirrorfour-api
              ];

              services.postgresql = {
                ensureDatabases = [ cfg.database.name ];
              };

              systemd.services.mirrorfour-api = {
                wantedBy = [ "multi-user.target" ];
                after = [ "postrgresql.service" ];
                environment = {
                  DATABASE_URL = "postgresql://${cfg.database.user}:${cfg.database.password}@localhost/${cfg.database.name}";
                  SESSION_KEY = "${cfg.sessionKey}";
                };
                serviceConfig = {
                  Type = "simple";
                  ExecStart = "${self.packages.${pkgs.system}.mirrorfour-api}/bin/mirrorfour-api";
                };
              };
            };
          };
        mirrorfour-api = default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            (
              { pkgs, ... }:
              {
                packages = with pkgs; [
                  rust-bin.stable.latest.default
                  rust-analyzer
                  taplo
                  sqlx-cli
                  pgformatter
                  yaml-language-server
                ];

                services.postgres.enable = true;

                env = {
                  DATABASE_URL = "postgresql:///mirrorfour";
                  SESSION_KEY = "bLHEx0Eyot2QO9C4RZQgJTVrKCUx5zwQFoHmLpGLqXg2WPy8ldMRAjlsPKLayHqlk91PP0IoQTGTYymCTN7aag==";
                };
              }
            )
          ];
        };

        packages = rec {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          devenv-test = self.devShells.${system}.default.config.test;

          default =
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.default;
              rustc = pkgs.rust-bin.stable.latest.default;
            }).buildRustPackage
              {
                pname = "mirrorfour-api";
                version = "0.1.0";

                src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;

                cargoLock.lockFile = ./Cargo.lock;

                nativeBuildInputs = with pkgs; [
                  curl
                ];
              };
          mirrorfour-api = default;
        };
      }
    );
}
