{pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/11cb3517b3af6af300dd6c055aeda73c9bf52c48.tar.gz") {}}:
pkgs.mkShell {
  packages = with pkgs; [
    alejandra
    cargo
    lolcat
    rustup
  ];
}
