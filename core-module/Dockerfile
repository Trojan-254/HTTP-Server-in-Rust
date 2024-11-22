# Use the official Rust image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Expose the desired port
EXPOSE 4228

# Command to run the application
CMD ["./target/release/core-module"]
