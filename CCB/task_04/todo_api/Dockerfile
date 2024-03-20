FROM rust:1.72 AS builder
COPY . .
RUN cargo build --release

FROM rockylinux:9
COPY --from=builder ./target/release/todo_api ./target/release/todo_api
ENTRYPOINT ["/target/release/todo_api"]