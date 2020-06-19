FROM rust:latest
WORKDIR /usr/src/downup

COPY Cargo.toml .
RUN cargo fetch

COPY src/ ./src/
RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/client"]
