FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

RUN apt-get update && apt-get install -y libgtk-4-dev libadwaita-1-dev fonts-noto-color-emoji flatpak flatpak-builder
RUN flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
RUN flatpak install flathub org.gnome.Platform//49 -y
RUN flatpak install flathub org.gnome.Sdk//49 -y