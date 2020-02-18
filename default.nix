{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  printerfacts = callPackage ./printerfacts.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "xena/printerfacts";
      tag = "latest";

      contents = [ pkg ];

      config = {
        Cmd = [ "/bin/printerfacts" ];
        Env = [ "ROCKET_PORT=5000" ];
        WorkingDir = "/";
      };
    };

in dockerImage printerfacts
