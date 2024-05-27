FROM rust:latest

WORKDIR /app/db_benchmark_stuff/

COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/

RUN cargo build --release

EXPOSE 9090

CMD ["cargo", "run", "--release"]