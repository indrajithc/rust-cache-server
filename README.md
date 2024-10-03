# Rust Cache Server

A high-performance cache server written in Rust using Actix-web. This server allows for storing, retrieving, and clearing cached data based on keys and tags.

## Features

- **Store Data**: Accepts JSON payloads to store key-value pairs along with tags.
- **Retrieve Data**: Allows retrieval of cached entries by their keys.
- **Clear Cache by Tags**: Supports clearing cache entries based on tags.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- `cargo` (comes with Rust).

### Installation

1. Clone the repository:

```
git clone https://github.com/yourusername/rust-cache-server.git
```

2. Change directory to the project folder:

```
cd rust-cache-server
```

3. Update dependencies:

```
cargo update
```

### Running the Server

To run the server, use the following command:

```
cargo run
```

The server will start on `http://localhost:5000`.

## API Endpoints

### Store Cache Entry

**POST** `/cache`

**Request Body**:
```json
{
  "key": "myKey",
  "value": "myValue",
  "tags": ["tag1", "tag2"]
}
