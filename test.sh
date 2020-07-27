
#!/usr/bin/env sh

# set -uo pipefail

# guard against nix shell
if [[ -z $IN_NIX_SHELL ]]
 then
  echo "test needs to be run from the nix shell"
  exit 1
fi

# move into the directory this script is in
cd "${0%/*}"

cargo fmt

# test as binary
cargo test -- --nocapture
