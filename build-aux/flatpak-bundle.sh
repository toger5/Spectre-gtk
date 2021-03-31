flatpak-builder --repo=repo .flatpak-app-dir build-aux/com.github.spectre.json --force-clean
flatpak build-bundle repo spectre.flatpak com.github.spectre