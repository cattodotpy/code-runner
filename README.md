# Code Runner

A secure, sandboxed API service for executing code in various programming languages.

## Security Features

- **Containerization**: Uses `hakoniwa` for lightweight containerization to isolate code execution
- **Namespace Isolation**: Implements multiple Linux namespace isolations (cgroup, IPC, UTS, network)
- **Seccomp Filtering**: Blocks dangerous system calls including:
    - mount/umount operations
    - network operations (socket, bind, connect, etc.)
    - system control operations (poweroff, reboot)
- **Resource Limiting**:
    - CPU time limits
    - Memory usage limits
    - Wall time (real-time) execution limits
- **Randomized Execution Directories**: Creates unique, random directories for each code execution
- **No Shell Access**: Executes programs directly without shell access

## Installation

### Prerequisites

- Rust toolchain (2024 edition)
- Linux environment (required for containerization features) (WSL or Docker are supported and tested.)
- (seccomp)[https://github.com/libseccomp-rs/libseccomp-rs#requirements]

### Building

```bash
cargo build --release
```

## Configuration

Create a `config.toml` file in the project root:

```toml
code_dir = "/path/to/execution/directory" # Directory where code will be executed
port = 8080 # Optional, defaults to 8080
address = "0.0.0.0" # Optional, defaults to 0.0.0.0

[languages.python3]
run = ["/bin/python3", "/box/program.py"]
extension = "py"

[languages.cpp-20]
compile = ["/bin/g++", "-o", "/box/program", "/box/program.cpp"]
run = ["/box/program"]
extension = "cpp"

# Add more language configurations as needed
```

## Running

```bash
./target/release/code-runner
```

The server will start on the configured address and port (default: 0.0.0.0:8080).

## API Usage

### Execute Code

```http
POST /code
```

Request body:

```json
{
	"language": "python3",
	"code": "print('Hello, world!')",
	"input": "",
	"time_limit": 2,
	"memory_limit": 67108864,
	"wall_time_limit": 5
}
```

Response:

```json
{
	"status": "Success",
	"stdout": "Hello, world!",
	"stderr": "",
	"runtime": 42,
	"memory_usage": 8192
}
```

### List Available Languages

```http
GET /languages
```

Response:

```json
{
	"languages": ["python3", "cpp-20"]
}
```

## Security Considerations

- Always run this service with minimal system privileges
- Do not expose this service directly to the internet without proper authentication
- Consider setting up resource quotas on the host system
- Regularly update dependencies to patch security vulnerabilities

## License

MIT
