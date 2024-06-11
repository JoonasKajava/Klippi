{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  languages.rust.enable = true;
  languages.javascript.enable = true;
  languages.typescript.enable = true;
}
