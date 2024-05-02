# Source: https://github.com/thoughtpolice/buck2-nix/blob/main/buck/nix/flake.nix
#
# "Development environment flake." This is used, along with direnv, to populate
# a shell with all the necessary setup to get things rolling in an incremental
# fashion.
{
  description = "md2pdf-service";

  # Flake inputs. These are the only external inputs we use to build the system
  # and describe all further build configuration.
  #
  # THIS LIST SHOULD NOT BE EXPANDED WITHOUT GOOD REASON. If you need to add
  # something, think hard about whether or not it can be achieved. Why? Because
  # every dependency that comes from elsewhere is a (potential) liability in the
  # quality control and security departments.
  #
  # The fact of the matter is that even the most mandatory input of all,
  # `nixpkgs`, already expose massive amount of surface area to the project and
  # downstream consumers. This is a good thing due to its versatility and
  # support, but it also means that we need to hedge our bets in other places.
  # Think of it like an investment: we already spent a good chunk of change, so
  # we don't want to spend too much more. If the option is "Bring in a 3rd party
  # dependency" or "Write 100 lines of Nix and stuff them in the repository",
  # the second one is almost always preferable.
  #
  # Furthermore, for the sake of maintainability and QA, we also make sure any
  # dependency has a consistent set of transitive dependencies.
  #
  # (Honestly, it'd be great if flake-utils could go away, but we need it anyway
  # for rust-overlay, alas.)
  #
  # Moral of the story: DO NOT EXPAND THIS INPUT LIST WITHOUT GOOD REASON.
  inputs = {
    # Nixpkgs (take the systems nixpkgs version)
    nixpkgs.url = "nixpkgs";

    # You can access packages and modules from different nixpkgs revs
    # at the same time. Here's an working example:
    nixpkgsStable.url = "github:nixos/nixpkgs/nixos-23.11";
    # Also see the 'stable-packages' overlay at 'overlays/default.nix'.

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs = {
        systems.follows = "systems";
      };
    };

    githooks = {
      url = "github:gabyx/githooks?dir=nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    systems.url = "github:nix-systems/default";
  };

  # [tag:custom-nix-config] Custom configuration. We use this to add our own
  # user level customization to `nix.conf`, primarily for the binary cache, and
  # is the primary reason we require the developer to be in 'trusted-users'.
  nixConfig = {
    # # see [ref:cache-url-warning]
    # extra-substituters = "https://buck2-nix-cache.aseipp.dev/";
    #
    # # one day, we won't need our own key when we can use [ref:ca-derivations]...
    # trusted-public-keys = "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= buck2-nix-preview.aseipp.dev-1:sLpXPuuXpJdk7io25Dr5LrE9CIY1TgGQTPC79gkFj+o=";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    githooks,
    ...
  }: let
    systems = with flake-utils.lib; [
      system.x86_64-linux
      system.aarch64-darwin
    ];
  in
    flake-utils.lib.eachSystem systems (system: let
      # The imported nixpkgs package set; all usages come from here with
      # overlays nicely applied.
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];

        # [tag:ca-derivations] One day, we'll enable content-addressable
        # derivations for all outputs here. This should significantly help any
        # and all toolchain support in a number of ways, primarily through:
        #
        #  - early cut-off optimization
        #  - self-authenticating paths (no more signing needed!)
        #
        # ideally, in a utopia, this would be the only way Nix worked in the
        # future, but it's too buggy for right now...
        #
        # XXX FIXME: enable this, one day...
        config.contentAddressedByDefault = false;
      };

      jobs = rec {
        packages = flake-utils.lib.flattenTree {
          buildbarn-vm = import ./buck/nix/bb/vm.nix {inherit pkgs;};
          build-container = import ./buck/nix/bb/container.nix {inherit pkgs;};
          buck2 = pkgs.callPackage ./buck/nix/buck2 {};
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # These are the global dependencies used for the whole project.
        # This could be
        nativeBuildInputs = {
          basic = with pkgs; [
            coreutils
            findutils
            gh
            git
            curl
            watchman # fs integration
            bash

            githooks.packages.${pkgs.system}.default
            packages.buck2

            rustToolchain
            reindeer
            cargo-watch
            lldb_16 # for lldb_vscode

            jq
            just
            tagref
            dasel
            parallel
            tilt
            kustomize
            sqlfluff # Linter

            python311Packages.isort
            python311Packages.black
          ];

          # Thinsg needed for local development.
          localDev = with pkgs; [
            k3s
            httpie
            podman
            dbeaver
          ];
        };

        # Things needed at runtime.
        buildInputs = with pkgs; [postgresql];

        shell = flake-utils.lib.flattenTree {
          # Add a convenient alias for 'buck bxl' on some scripts. note that
          # the 'bxl' cell location can be changed in .buckconfig without
          # changing the script
          bxl = pkgs.writeShellScriptBin "bxl" ''
            exec ${jobs.packages.buck2}/bin/buck bxl "bxl//top.bxl:$1" -- "''${@:2}"
          '';

          # A convenient script for starting a vm, that can then run buildbarn
          # in an isolated environment
          start-buildbarn-vm = pkgs.writeShellScriptBin "start-buildbarn-vm" ''
            export NIX_DISK_IMAGE=$(buck root -k project)/buildbarn-vm.qcow2
            if ! [ -f "$NIX_DISK_IMAGE" ]; then
                ${pkgs.qemu-utils}/bin/qemu-img -- create -f qcow2 $NIX_DISK_IMAGE 20G
            fi

            VM_BIN=${packages.buildbarn-vm}
            exec $VM_BIN/bin/run-nixos-vm "$@"
          '';
        };

        # The default Nix shell. This is populated by direnv and used for the
        # interactive console that a developer uses when they use buck2
        # et cetera.
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs.basic ++ nativeBuildInputs.localDev ++ buildInputs ++ (builtins.attrValues shell);
        };

        # Formatter for your nix files, available through 'nix fmt'
        # Other options beside 'alejandra' include 'nixpkgs-fmt'
        formatter = pkgs.alejandra;
      };

      # Flatten the hierarchy; mostly used to ensure we build everything...
      flatJobs = flake-utils.lib.flattenTree {
        packages = jobs.packages // {recurseForDerivations = true;};
        shell = jobs.shell // {recurseForDerivations = true;};
      };
    in {
      inherit (jobs) devShells;
      inherit (jobs) formatter;

      packages =
        rec {
          # By default, build all the packages in the tree when just running
          # 'nix build'; useful for various development tasks, since it ensures
          # a fully 'clean' closure builds. But we obviously don't use it for
          # devShells...
          default = world;

          # List of all attributes in this whole flake; useful for the cache
          # upload scripts, and also CI and other things probably...
          attrs = pkgs.writeText "attrs.txt" (pkgs.lib.concatStringsSep "\n" (["world"] ++ (builtins.attrNames flatJobs)));

          # XXX FIXME (aseipp): unify this with 'attrs' someday...
          world = pkgs.writeText "world.json" (builtins.toJSON {
            buildPackages = jobs.packages;
            shellPackages = jobs.shell;
          });

          # Merge in flatJobs, so that when we do things like 'nix flake show'
          # or try to list and build all attrs, we can see all the packages and
          # toolchains, et cetera.
        }
        // flatJobs;
    });
}
