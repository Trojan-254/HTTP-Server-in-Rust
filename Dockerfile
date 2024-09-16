# Use the official Rust image as the base image
FROM rust:1.71

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock manifest files
COPY core-module/Cargo.toml core-module/Cargo.lock ./

# Create a dummy file to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Copy the rest of the source code
COPY core-module/ ./

# Build the actual application
RUN cargo build --release

# Set the entrypoint to your application binary
# ENTRYPOINT ["/HTTP-Server-in-Rust/core-module/target/release/core-module"]
ENTRYPOINT [ "./target/release/core-module" ]

# Expose any ports the app uses
EXPOSE 4228
