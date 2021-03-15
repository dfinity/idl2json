{ nixpkgs ?
  import (builtins.fetchTarball {
    # Descriptive name to make the store path easier to identify
    name = "nixos-unstable-2020-09-24";
    # Commit hash for nixos-unstable as of 2018-09-12
    # `git ls-remote https://github.com/nixos/nixpkgs-channels nixos-unstable`
    url = "https://github.com/nixos/nixpkgs/archive/1179840f9a88b8a548f4b11d1a03aa25a790c379.tar.gz";
    # Hash obtained using `nix-prefetch-url --unpack <url>`
    sha256 = "00jy37wj04bvh299xgal2iik2my9l0nq6cw50r1b2kdfrji8d563";
  }) {}
}:
let
  inherit (nixpkgs) pkgs;

  nixPackages = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.clippy
    pkgs.libiconv
  ];
in
pkgs.stdenv.mkDerivation {
  name = "env";
  buildInputs = nixPackages;
}
