{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  helloworld = import ./helloworld.nix { inherit sources pkgs; };

  name = "nasirhm/hello-nix-rust";
  tag = "latest";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ helloworld ];

  config = {
    Cmd = [ "/bin/helloworld" ];
    Env = [ "ROCKET_PORT=5000" ];
    WorkingDir = "/";
  };
}
