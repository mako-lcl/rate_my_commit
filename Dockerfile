# Use an existing Rust image as the base
FROM rust:bookworm as builder

# Set the working directory
WORKDIR /app

# Copy the application files into the image
COPY Cargo.toml  .
COPY Cargo.lock .
COPY src ./src

# Build the application in release mode
RUN cargo build --release

FROM debian:bookworm-slim

# Install the runtime dependencies (libssl1.1 should provide libssl.so.1.1)
#RUN apt-get update && apt upgrade && \
#    apt-get install -y libssl1.1 ca-certificates && \
#    rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt install -y openssl

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rate_my_commit /usr/local/bin/rate_my_commit

# Set the command to run the binary
CMD ["/usr/local/bin/rate_my_commit"]