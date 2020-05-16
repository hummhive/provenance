with import <nixpkgs> { };
stdenv.mkDerivation {
 name = "rust-env";
 nativeBuildInputs = [
  rustc cargo
 ];

 RUST_BACKTRACE = 1;

 shellHook = ''
  set -o allexport
  source .env
  set +o allexport
 '';
}
