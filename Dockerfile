FROM rust:latest
WORKDIR /usr/src/downup

# cached crates.io
COPY Cargo.toml .
RUN cargo fetch

# cached deps build
#RUN mkdir -p src/bin
#RUN echo "fn main() {}" >> src/bin/client.rs
#RUN echo "fn main() {}" >> src/bin/server.rs
#RUN echo "" >> src/mod.rs
#RUN cargo build --release

# build src
#RUN rm -r src

COPY src/ ./src/
RUN cargo build --release

# install bin
RUN cargo install --path .

