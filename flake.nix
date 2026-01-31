{
  description = "ASUS laptop fan state control utility";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = system: nixpkgs.legacyPackages.${system};
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = self.packages.${system}.fan_state;
          fan_state = pkgs.rustPlatform.buildRustPackage {
            pname = "fan_state";
            version = "1.0.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
              dbus
            ];

            meta = with pkgs.lib; {
              description = "Set the fan state on the ZenBook S 16 UM5606 and Vivobook M5606";
              homepage = "https://github.com/ThatOneCalculator/asus-5606-fan-state";
              license = licenses.mit;
              maintainers = [ ];
              platforms = [ "x86_64-linux" "aarch64-linux" ];
              mainProgram = "fan_state";
            };
          };

          fan_state-no-dbus = self.packages.${system}.fan_state.override {
            buildInputs = [ ];
          } // {
            buildNoDefaultFeatures = true;
          };
        });

      nixosModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.services.asus-fan-state;
        in
        {
          options.services.asus-fan-state = {
            enable = lib.mkEnableOption "ASUS fan state control";

            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.system}.fan_state;
              description = "The fan_state package to use";
            };
          };

          config = lib.mkIf cfg.enable {
            environment.systemPackages = [ cfg.package ];

            systemd.services.asus-fan-permissions = {
              description = "Set permissions for ASUS fan control";
              after = [ "sys-kernel-debug.mount" ];
              wantedBy = [ "multi-user.target" ];
              unitConfig = {
                ConditionPathExists = "/sys/kernel/debug/asus-nb-wmi";
              };
              serviceConfig = {
                Type = "oneshot";
                RemainAfterExit = true;
                ExecStart = pkgs.writeShellScript "asus-fan-permissions" ''
                  chmod 755 /sys/kernel/debug
                  chmod 666 /sys/kernel/debug/asus-nb-wmi/dev_id
                  chmod 666 /sys/kernel/debug/asus-nb-wmi/ctrl_param
                '';
              };
            };
          };
        };

      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rust-analyzer
              clippy
              rustfmt
              pkg-config
              dbus
            ];

            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        });
    };
}
