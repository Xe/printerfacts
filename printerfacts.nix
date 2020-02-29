{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  gruvbox-css = import sources.gruvbox-css { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  xepkgs = import sources.xepkgs { inherit sources pkgs; };
  src = xepkgs.srcNoTarget ./.;
  pfacts = naersk.buildPackage {
    inherit src;
    remapPathPrefix = true;
  };
in pkgs.stdenv.mkDerivation {
  name = pfacts.name;
  version = pfacts.version;

  inherit src;
  phases = "installPhase";

  installPhase = ''
    mkdir -p $out/public

    cp -rf $src/templates $out/templates

    cp -rf ${pfacts}/bin $out/bin
    cp -rf ${gruvbox-css}/gruvbox.css $out/public/gruvbox.css
  '';
}
