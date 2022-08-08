FROM rust:buster as builder

RUN apt update && apt install libzmq3-dev -y
WORKDIR "/proj"
COPY "." "./"
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update && apt install libzmq3-dev -y
WORKDIR "/app"
COPY --from=builder "/proj/target/release" "./"
ENTRYPOINT ["./AnnaDB"]

