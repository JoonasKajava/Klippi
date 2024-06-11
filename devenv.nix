{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [libarchive openssl libsoup cairo gdk-pixbuf pango atkmm gtk3 webkitgtk];

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
