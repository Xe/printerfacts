{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  printerfacts = import ./printerfacts.nix { inherit pkgs sources; };
  xepkgs = import sources.xepkgs { inherit pkgs sources; };

  name = "xena/printerfacts";
  tag = "latest";

in xepkgs.dockerImage {
  inherit name tag;
  contents = [ printerfacts ];

  config = {
    Cmd = [ "/bin/printerfacts" ];
    Env = [ "ROCKET_PORT=5000" ];
  };
}
