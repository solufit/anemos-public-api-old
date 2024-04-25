FROM rust:1.77.2-slim-bookworm

# COPY FILE
COPY ./public-api/ /src/

# SET WORKDIR
WORKDIR /src/

# COMPILE SOURCE CODE
RUN cargo build --release

# RUN
ENTRYPOINT [ "cargo", "run", "--release" ]

