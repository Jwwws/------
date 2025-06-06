use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

const BUFFER_SIZE: usize = 4096;

struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    fn parse(request_str: &str) -> Option<HttpRequest> {
        let lines: Vec<&str> = request_str.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line_parts.len() < 3 {
            return None;
        }

        let method = first_line_parts[0].to_string();
        let path = first_line_parts[1].to_string();
        let mut headers = HashMap::new();

        for line in &lines[1..] {
            if line.trim().is_empty() {
                break;
            }
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_lowercase();
                let value = line[colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        Some(HttpRequest { method, path, headers })
    }
}

async fn handle_request(request: &HttpRequest) -> String {
    match request.path.as_str() {
        "/" => {
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Welcome to Rust Web Server!</h1><p>Async version with Tokio</p>".to_string()
        }
        "/api/hello" => {
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello from Async Rust!\"}".to_string()
        }
        "/slow" => {
            // 模拟异步慢请求
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Async Slow Response</h1>".to_string()
        }
        _ => {
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 - Page Not Found</h1>".to_string()
        }
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; BUFFER_SIZE];
    
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => {
            let request_str = std::str::from_utf8(&buffer[..bytes_read])
                .unwrap_or("");
            
            if let Some(request) = HttpRequest::parse(request_str) {
                let response = handle_request(&request).await;
                let _ = stream.write_all(response.as_bytes()).await;
            }
        }
        Err(e) => eprintln!("Failed to read from client: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8082").await?;
    println!("Async server listening on 127.0.0.1:8082");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}