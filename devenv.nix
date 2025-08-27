{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

let
  md2pk-rs = config.languages.rust.import ./. { };
in
{
  # https://devenv.sh/languages/
  languages.rust.enable = true;

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  # https://devenv.sh/packages/
  packages = [ md2pk-rs ];

  # https://devenv.sh/outputs/
  outputs = {
    inherit md2pk-rs;
  };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    cargo test
  '';
}
