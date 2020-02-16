let
  sources = import ./sources.nix;
  dhall-yaml = import ./dhall-yaml.nix;
  rust = import ./rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    dhall-yaml
    kubectl
    doctl
  ];
}
