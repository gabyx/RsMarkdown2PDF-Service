let
  overlays = [
    (import <overlay-rust>)
  ];
  config = { };
in with import <buckpkgs> { inherit config overlays; }; (
pkgs.oil
) /* EOF */
