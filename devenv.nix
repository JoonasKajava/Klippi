{pkgs, ...}: {
  packages = with pkgs;
    [
      libarchive
    ]
    ++ [
      at-spi2-atk
      atkmm
      cairo
      gdk-pixbuf
      glib
      gtk3
      harfbuzz
      librsvg
      libsoup_3
      pango
      webkitgtk_4_1
      openssl
    ];

  scripts.bump.exec = ''
    cz bump --changelog --changelog-to-stdout > CURRENT_CHANGELOG.md;
  '';
  languages = {
    rust.enable = true;
    javascript = {
      enable = true;
      npm = {
        enable = true;
        install.enable = true;
      };
    };
    typescript.enable = true;
  };
}
