FROM rust:1.77.2-slim-bookworm

# Installl pkg-config and libssl-dev
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# remove apt cache
RUN rm -rf /var/lib/apt/lists/*

# COPY FILE
COPY . /src/

# SET WORKDIR
WORKDIR /src/public-api-collector

# COMPILE SOURCE CODE
RUN cargo build --release

# RUN
CMD [ "cargo", "run", "--release" ]

