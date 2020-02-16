{ sources ? import ./nix/sources.nix }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  gruvbox-css = import sources.gruvbox-css { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;
  pfacts = naersk.buildPackage { inherit src; };
in pkgs.stdenv.mkDerivation {
  name = pfacts.name;
  version = pfacts.version;

  inherit src;
  phases = "installPhase";

  installPhase = ''
    mkdir -p $out/bin $out/public

    cp -rf $src/templates $out/templates

    cp -rf ${pfacts}/bin $out/bin
    cp -rf ${gruvbox-css}/gruvbox.css $out/public/gruvbox.css
  '';
}
