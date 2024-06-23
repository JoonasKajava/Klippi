{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = with pkgs;
    [
      openssl
      libsoup
      cairo
      gdk-pixbuf
      pango
      atkmm
      gtk3
      webkitgtk
    ]
    ++ (with pkgs.nodePackages_latest; [
      typescript-language-server
    ])
    ++ (with pkgs.gst_all_1; [
      gst-libav
      gst-plugins-bad
      gst-plugins-base
      gst-plugins-good
      gst-plugins-ugly
      gst-vaapi
      gstreamer
    ]);

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
