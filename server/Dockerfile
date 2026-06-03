# Build
FROM rust:1.96.0 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs
RUN cargo build --release

# Now copy the actual source code and build the application
COPY src ./src
# Use touch to uncache main.rs
RUN touch src/main.rs && cargo build --release --bin server

# Runtime
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/server .

EXPOSE 8080

CMD ["./server"]