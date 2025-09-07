# HTTP Client from Scratch in Rust 🚀

A minimal HTTP client implementation built from scratch using only Rust's standard library and raw TCP sockets. This project demonstrates the fundamentals of HTTP protocol communication without relying on high-level HTTP libraries.

## 🎯 Project Overview

This HTTP client provides a deep understanding of:

- HTTP protocol structure (request lines, headers, status codes)
- TCP socket programming
- Network error handling
- The stateless nature of HTTP communication

## ✨ Features

### Core HTTP Methods

- ✅ **GET** - Retrieve data from servers
- ✅ **POST** - Send data to servers
- ✅ **PUT** - Update resources
- ✅ **DELETE** - Remove resources

### Error Handling Categories

- **Network Errors** - Connection failures, timeouts, DNS issues
- **HTTP Errors** - 404 Not Found, 500 Server Error, etc.
- **Protocol Errors** - Malformed responses, invalid data

### Basic Usage

```bash
cargo run
```

## 📝 Code Examples

### Simple GET Request

```rust
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get("httpbin.org", "/get", None)?;
    println!("Status: {}", response.status_line);
    println!("Body: {}", response.body);
    Ok(())
}
```

### POST with JSON Data

```rust
let json_data = r#"{"name": "John", "age": 30}"#;
let response = post("httpbin.org", "/post", json_data, None)?;
println!("Response: {}", response.body);
```

### Custom Headers

```rust
let mut headers = HashMap::new();
headers.insert("Authorization".to_string(), "Bearer token123".to_string());
headers.insert("User-Agent".to_string(), "MyClient/1.0".to_string());

let response = get("api.example.com", "/protected", Some(headers))?;
```

## 🏗️ Architecture

### Core Components

#### `HttpResponse` Struct

```rust
struct HttpResponse {
    status_line: String,  // "HTTP/1.1 200 OK"
    status_code: u16,     // 200, 404, 500, etc.
    headers: Vec<String>, // Response headers
    body: String,         // Response content
}
```

#### `HttpError` Enum

```rust
enum HttpError {
    NetworkError(String),              // Connection issues
    InvalidResponse(String),           // Protocol violations
    HttpError { code: u16, message: String }, // HTTP status errors
}
```

#### Core Functions

- `send_request()` - Unified request handler for all HTTP methods
- `parse_response()` - Parses raw HTTP responses into structured data
- `get()`, `post()`, `put()`, `delete()` - Convenience functions

## 🧪 Testing

The project includes comprehensive tests for:

### Successful Scenarios

- ✅ Basic GET requests
- ✅ POST with JSON payloads
- ✅ Custom header handling

### Error Scenarios

- ❌ 404 Not Found responses
- ❌ Network connectivity issues
- ❌ DNS lookup failures
- ❌ Connection timeouts
