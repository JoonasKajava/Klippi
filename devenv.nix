{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs; [openssl libsoup cairo gdk-pixbuf pango atkmm gtk3 webkitgtk];

  scripts.bump.exec = ''
    cz bump --changelog --changelog-to-stdout > CURRENT_CHANGELOG.md;
  '';

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
