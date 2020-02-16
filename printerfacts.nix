{ sources ? import ./nix/sources.nix }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  gruvbox-css = import sources.gruvbox-css { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
in
naersk.buildPackage {
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;

  postInstall = ''
    cp -rf $src/public $out/public
    cp -rf $src/templates $out/templates

    cp -rf ${gruvbox-css}/gruvbox.css $out/public/gruvbox.css
  '';
}
