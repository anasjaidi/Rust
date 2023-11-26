FROM rust:latest

WORKDIR /api

COPY "./Cargo.toml" "./"

RUN cargo build --release 

COPY ./ ./

CMD [ "./target/debug/api" ]