let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs {
    config = {};
    overlays = [];
  };
in
  pkgs.mkShellNoCC {
    packages = with pkgs; [
      cowsay
      lolcat
    ];

    ENV_VAR = "Hello world";

    shellHook = ''
      echo $ENV_VAR | cowsay | lolcat
    '';
  }
