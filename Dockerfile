FROM debian:bullseye-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    gcc \
    g++ \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    libseccomp-dev \
	&& rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create app directories
WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Create the judger directory (from config.toml)
RUN mkdir -p /root/judger

# Expose the port (adjust if your application uses a different port)
EXPOSE 8080

# Run the application
CMD ["./target/release/code-runner"]
