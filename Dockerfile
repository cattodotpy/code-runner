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

RUN cargo build --release

EXPOSE $PORT

# Run the application
CMD ["./target/release/code-runner"]
