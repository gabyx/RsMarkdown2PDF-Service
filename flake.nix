{
  description = "Homepage Dev";

  nixConfig = {
    substituters = [
      # Replace the official cache with a mirror located in China
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
    # Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # You can access packages and modules from different nixpkgs revs
    # at the same time. Here's an working example:
    nixpkgsStable.url = "github:nixos/nixpkgs/nixos-23.05";
    # Also see the 'stable-packages' overlay at 'overlays/default.nix'.

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    nixpkgsStable,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem
    # Creates an attribute map `{ devShells.<system>.default = ...}`
    # by calling this function:
    (
      system: let
        # From: https://nixos.wiki/wiki/Rust
        # Instantiate the pkgs:
        # Import the nixpkgs and call its function with `{ system = system }`
        pkgs = import nixpkgs {
          inherit system;
        };

        # Create the shell.
        shell = mkShell rec {
          buildInputs = with pkgs; [
            clang
            # Replace llvmPackages with llvmPackages_X,
            # where X is the latest LLVM version (at the time of writing, 16)
            llvmPackages.bintools
            rustup
          ];

          RUSTUP_TOOLCHAIN = builtins.fromTOML (builtins.readFile (self + /rust-toolchain.toml));

          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];

          # Add precompiled library to rustc search path
          RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ];

          # Add glibc, clang, glib and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
            # Includes with normal include path
            (builtins.map (a: ''-I"${a}/include"'') [
              # add dev libraries here (e.g. pkgs.libvmi.dev)
              pkgs.glibc.dev
            ])
            # Includes with special directory paths
            ++ [
              ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
            ];

          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTUP_TOOLCHAIN-x86_64-unknown-linux-gnu/bin/
          '';
        };
      in {
        devShells.default = shell;
      }
    );
}
