# Offizielles Rust-Image mit Cargo & rustc
FROM rust:latest

# Installiere Debug-Tools
RUN apt-get update && apt-get install -y gdb lldb

# Setze das Arbeitsverzeichnis im Container
WORKDIR /usr/src/app

# Setze den Container-User (vermeidet Root-Probleme)
RUN useradd -m rustdev && chown -R rustdev /usr/src/app
USER rustdev

# Standard-Command: Rust-Version anzeigen
#CMD ["rustc", "--version"]
CMD ["cargo", "test"]