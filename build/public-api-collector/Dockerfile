FROM rust:1.77.2-slim-bookworm

# COPY FILE
COPY . /src/

# SET WORKDIR
WORKDIR /src/public-api-collector

# COMPILE SOURCE CODE
RUN cargo build --release

# RUN
CMD [ "cargo", "run", "--release" ]

