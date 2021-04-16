{ pkgs ?
  import <nixpkgs> {}
}:
with pkgs;
rustPlatform.buildRustPackage rec {
  name = "idl2json-${version}";
  version = "0.1.0";
  src = lib.sources.cleanSource ./.;
  buildInputs = [ libiconv ];

  checkPhase = "";
  cargoSha256 = "sha256:1hmxwmb3x207rhwi6ddnfvhhsc5i3asdxs6rql4xnjyx1p5nyf19";
  meta = with stdenv.lib; {
    description = "Parse idl output from dfx.";
    homepage = https://github.com/dfinity-lab/idl2json;
    license = licenses.mit;
    maintainers = [ maintainers.tailhook ];
    platforms = platforms.all;
  };
}
