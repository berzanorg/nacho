# Use Debian Bookworm with Node.js as the base image.
FROM node:bookworm

# Install dependencies.
RUN apt update && apt install -y \
    git \
    fish \
    opam

# Set the shell environment variable and create a configuration file for Fish.
ENV SHELL=/usr/bin/fish
RUN mkdir "$HOME/.config"
RUN mkdir "$HOME/.config/fish"

# Install and setup OCaml.
RUN opam init --yes
RUN opam install ocaml-lsp-server odoc ocamlformat utop --yes
RUN opam config env > "$HOME/.config/fish/config.fish"

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y  
RUN "$HOME/.cargo/bin/rustup" component add rustfmt

# Install and configure pnpm.
RUN corepack enable pnpm
RUN pnpm config set store-dir "$HOME/.pnpm-store"
