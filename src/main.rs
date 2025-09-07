use std::net::TcpStream;
use std::io::{Write, Read};
use std::collections::HashMap;
use std::time::Duration;

// Enhanced response struct with status code parsing
#[derive(Debug)]
struct HttpResponse {
    status_line: String,
    status_code: u16,
    headers: Vec<String>,
    body: String,
}

// Custom error types for better error handling
#[derive(Debug)]
enum HttpError {
    NetworkError(String),
    InvalidResponse(String),
    HttpError { code: u16, message: String },
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HttpError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            HttpError::HttpError { code, message } => write!(f, "HTTP {} error: {}", code, message),
        }
    }
}

impl std::error::Error for HttpError {}

fn send_request(
    method: &str,
    host: &str, 
    path: &str, 
    body: Option<&str>,
    custom_headers: Option<HashMap<String, String>>
) -> Result<HttpResponse, HttpError> {
    // timeout for unresponsive connection
    let mut stream = TcpStream::connect(format!("{}:80", host))
        .map_err(|e| HttpError::NetworkError(format!("Failed to connect to {}: {}", host, e)))?;
    
    stream.set_read_timeout(Some(Duration::from_secs(10)))
        .map_err(|e| HttpError::NetworkError(format!("Failed to set timeout: {}", e)))?;
    
    // Build request line
    let mut request = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", method, path, host);
    
    // Content-Length and Content-Type if body exists
    if let Some(body_content) = body {
        request.push_str(&format!("Content-Length: {}\r\n", body_content.len()));
        request.push_str("Content-Type: application/json\r\n");
    }
    
    // custom headers if provided
    if let Some(headers) = custom_headers {
        for (key, value) in headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
    }
    
    request.push_str("Connection: close\r\n\r\n");
    
    // body if it exists
    if let Some(body_content) = body {
        request.push_str(body_content);
    }
    
    stream.write_all(request.as_bytes())
        .map_err(|e| HttpError::NetworkError(format!("Failed to send request: {}", e)))?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)
        .map_err(|e| HttpError::NetworkError(format!("Failed to read response: {}", e)))?;
    
    parse_response(&response)
}

fn parse_response(response: &str) -> Result<HttpResponse, HttpError> {
    let mut lines = response.lines();
    
    let status_line = lines.next()
        .ok_or_else(|| HttpError::InvalidResponse("Empty response".to_string()))?
        .to_string();
    
    // Parse status code
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|code| code.parse::<u16>().ok())
        .ok_or_else(|| HttpError::InvalidResponse("Invalid status line".to_string()))?;
    
    let mut headers = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        headers.push(line.to_string());
    }
    
    let body = lines.collect::<Vec<_>>().join("\n");

        // Check for HTTP errors
    if status_code >= 400 {
        return Err(HttpError::HttpError {
            code: status_code,
            message: format!("Server returned error: {}", status_line),
        });
    }
    
    
    let response = HttpResponse {
        status_line,
        status_code,
        headers,
        body,
    };
    

    Ok(response)
}

// Update convenience functions to use new error type
fn get(host: &str, path: &str, headers: Option<HashMap<String, String>>) -> Result<HttpResponse, HttpError> {
    send_request("GET", host, path, None, headers)
}

fn post(host: &str, path: &str, body: &str, headers: Option<HashMap<String, String>>) -> Result<HttpResponse, HttpError> {
    send_request("POST", host, path, Some(body), headers)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Test successful request
    println!("=== Successful GET ===");
    match get("httpbin.org", "/get", None) {
        Ok(response) => println!("✅ Status: {} (Code: {})", response.status_line, response.status_code),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    Test 404 error
    println!("\n=== Testing 404 Error ===");
    match get("httpbin.org", "/nonexistent", None) {
        Ok(response) => println!("✅ Status: {}", response.status_line),
        Err(e) => println!("❌ Expected error: {}", e),
    }
    
    Test network error
    println!("\n=== Testing Network Error ===");
    match get("nonexistent-host-12345.com", "/", None) {
        Ok(response) => println!("✅ Status: {}", response.status_line),
        Err(e) => println!("❌ Expected error: {}", e),
    }

    test_api()?;
    
    Ok(())
    
}

