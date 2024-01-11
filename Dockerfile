FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN cargo install diesel_cli_ext

CMD ["cargo", "watch", "--why", "--", "echo"]