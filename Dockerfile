FROM rust:1.73-bookworm

# User from environment
ARG USER=test
ARG USER_UID=1000
ARG USER_GID=1000

# Create user
RUN groupadd --gid $USER_GID $USER \
    && useradd --uid $USER_UID --gid $USER_GID -m $USER


RUN apt update && apt install -y libclang-dev build-essential
