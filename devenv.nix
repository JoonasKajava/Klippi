{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [pyright libarchive openssl libsoup cairo gdk-pixbuf pango atkmm gtk3 webkitgtk];

  languages.python = {
    enable = true;
    venv.enable = true;
  };

  languages.rust.enable = true;
  languages.javascript = {
    enable = true;
    npm = {
      enable = true;
      install.enable = true;
    };
  };
  languages.typescript.enable = true;
}
