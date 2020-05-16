with import <nixpkgs> { };
stdenv.mkDerivation {
 name = "rust-env";
 nativeBuildInputs = [
  rustc cargo
 ];

 RUST_BACKTRACE = 1;

 shellHook = ''
  # source the .env file for environment variables
  set -a; . .env; set +a
 '';
}
