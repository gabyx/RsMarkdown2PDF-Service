let
  overlays = [
    (import <overlay-rust>)
  ];
  config = { };
in with import <buckpkgs> { inherit config overlays; }; (
pkgs.rust-bin.stable."1.67.0".default
) /* EOF */
