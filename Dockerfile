FROM rust:1.66.1

WORKDIR /usr/src/onefile
COPY . .

RUN cargo build --release

RUN mv ./target/release/onefile ./
RUN rm -rf ./target