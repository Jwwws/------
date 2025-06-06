use nix::sys::socket::{socket, bind, listen, accept, AddressFamily, SockType, SockFlag, SockaddrIn};
use nix::unistd::{read, write, close};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;

const BUFFER_SIZE: usize = 4096;
const BACKLOG: usize = 10;

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

fn handle_request(request: &HttpRequest) -> String {
    match request.path.as_str() {
        "/" => {
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Welcome to Rust Web Server!</h1><p>Single-threaded version</p>".to_string()
        }
        "/api/hello" => {
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello from Rust!\"}".to_string()
        }
        _ => {
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 - Page Not Found</h1>".to_string()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建socket
    let server_fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )?;

    // 绑定地址
    let addr = SockaddrIn::new(127, 0, 0, 1, 8080);
    bind(server_fd, &addr)?;

    // 开始监听
    listen(server_fd, BACKLOG)?;
    println!("Single-threaded server listening on 127.0.0.1:8080");

    loop {
        // 接受连接
        match accept(server_fd) {
            Ok(client_fd) => {
                // 读取请求
                let mut buffer = [0u8; BUFFER_SIZE];
                match read(client_fd, &mut buffer) {
                    Ok(bytes_read) => {
                        let request_str = std::str::from_utf8(&buffer[..bytes_read])
                            .unwrap_or("");
                        
                        if let Some(request) = HttpRequest::parse(request_str) {
                            let response = handle_request(&request);
                            let _ = write(client_fd, response.as_bytes());
                        }
                    }
                    Err(e) => eprintln!("Failed to read from client: {}", e),
                }
                let _ = close(client_fd);
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}