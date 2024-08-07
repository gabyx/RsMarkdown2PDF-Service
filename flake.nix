{
  description = "Md2PDF-Service";

  nixConfig = {
    substituters = [
      # Add here some other mirror if needed.
      "https://cache.nixos.org/"
    ];
    extra-substituters = [
      # Nix community's cache server
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    # Nixpkgs (take the systems nixpkgs version)
    nixpkgs.url = "nixpkgs";

    # You can access packages and modules from different nixpkgs revs
    # at the same time. Here's an working example:
    nixpkgsStable.url = "github:nixos/nixpkgs/nixos-23.11";
    # Also see the 'stable-packages' overlay at 'overlays/default.nix'.

    githooks = {
      url = "github:gabyx/githooks?dir=nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    nixpkgsStable,
    rust-overlay,
    githooks,
    ...
  } @ inputs: let
    # Supported systems for your flake packages, shell, etc.
    systems = [
      "x86_64-linux"
      "aarch64-darwin"
    ];

    # This is a function that generates an attribute by calling a function you
    # pass to it, with the correct `system` and `pkgs` as arguments.
    forAllSystems = func: nixpkgs.lib.genAttrs systems (system: func system nixpkgs.legacyPackages.${system});
  in {
    # Formatter for your nix files, available through 'nix fmt'
    # Other options beside 'alejandra' include 'nixpkgs-fmt'
    formatter = forAllSystems (system: pkgs: pkgs.alejandra);

    devShells = forAllSystems (
      system: legacyPkgs: let
        overlays = [(import rust-overlay)];

        # Import nixpkgs and load it into pkgs.
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Things needed only at compile-time.
        nativeBuildInputsBasic = with pkgs; [
          rustToolchain
          cargo-watch
          lldb_16 # for lldb_vscode

          just
          dasel
          parallel
          tilt
          kustomize
          sqlfluff # Linter

          python311Packages.isort
          python311Packages.black
        ];

        # Things needed only at compile-time.
        nativeBuildInputsDev = with pkgs; [
          coreutils
          findutils

          curl
          jq
          bash
          githooks.packages.${pkgs.system}.default
        ];

        nativeBuildInputsLocalDev = with pkgs; [
          k3s
          httpie
          podman
          dbeaver
        ];

        # Things needed at runtime.
        buildInputs = with pkgs; [postgresql];
      in {
        default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputsBasic ++ nativeBuildInputsDev ++ nativeBuildInputsLocalDev;
        };

        ci = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputsBasic ++ nativeBuildInputsDev;
        };
      }
    );
  };
}
