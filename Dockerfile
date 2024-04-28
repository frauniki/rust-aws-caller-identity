FROM rust:1.77-bullseye as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /app/target/release/rust-aws-caller-identity /

ENTRYPOINT ["/rust-aws-caller-identity"]
