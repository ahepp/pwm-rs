FROM debian:bullseye-slim

RUN dpkg-reconfigure debconf -f noninteractive -p critical \
    && echo 'root:root' | chpasswd \
    && groupadd -g 1000 dev \
    && useradd -m -u 1000 -g dev dev

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        clang \
        curl \
        libssl-dev \
        pkg-config

USER dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    && cargo install cargo-watch

ENV PATH /home/dev/.cargo/bin:$PATH
