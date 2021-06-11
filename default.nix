{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { }, makeWrapper ? pkgs.makeWrapper }:
let
  srcNoTarget = dir:
    builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    dir;
  gruvbox-css = import sources.gruvbox-css { };
  naersk = pkgs.callPackage sources.naersk { };
  src = srcNoTarget ./.;
  pfacts = naersk.buildPackage {
    inherit src;
    remapPathPrefix = true;
  };
in pkgs.stdenv.mkDerivation {
  name = pfacts.name;
  version = pfacts.version;

  inherit src;
  phases = "installPhase";

  buildInputs = [ makeWrapper ];

  installPhase = ''
    mkdir -p $out/public

    cp -rf $src/templates $out/templates

    cp -rf ${pfacts}/bin $out/bin
    cp -rf ${gruvbox-css}/gruvbox.css $out/public/gruvbox.css
  '';
}
