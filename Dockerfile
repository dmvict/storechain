FROM rust:1.73-bookworm

# User from environment
ARG USER=test
ARG USER_UID=1000
ARG USER_GID=1000

# Create user
RUN groupadd --gid $USER_GID $USER \
    && useradd --uid $USER_UID --gid $USER_GID -m $USER


RUN apt update && apt install -y libclang-dev build-essential

# download and install tendermint
RUN curl -L https://github.com/tendermint/tendermint/releases/download/v0.34.21/tendermint_0.34.21_linux_amd64.tar.gz > /tmp/tendermint.tar.gz
RUN tar -xvf /tmp/tendermint.tar.gz -C /usr/bin tendermint

# install sqlx utility to setup database
RUN cargo install sqlx-cli
