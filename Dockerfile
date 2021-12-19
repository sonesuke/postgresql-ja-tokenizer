FROM rust:1.57.0-bullseye

# install modules for cargo-pgx
RUN apt-get update && apt-get install -y \
    locales \
    sudo \
    libclang-dev \
    file \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN sed -i -E 's/# (en_US.UTF-8)/\1/' /etc/locale.gen && locale-gen

# add user for postgresql
ARG USERNAME=user
ARG GROUPNAME=user
ARG UID=1000
ARG GID=1000
ARG PASSWORD=user
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID -G sudo $USERNAME && \
    echo $USERNAME:$PASSWORD | chpasswd && \
    echo "$USERNAME   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
USER $USERNAME
WORKDIR /home/$USERNAME/

ENV LC_ALL=en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV USER=$USERNAME

# install cargo-pgx
RUN cargo install cargo-pgx
RUN cargo pgx init

# install formatter
RUN rustup component add rustfmt