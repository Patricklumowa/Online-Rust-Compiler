use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Json},
    http::Method,
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::{self, Stream, StreamExt}};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, process::Stdio};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio_util::codec::{FramedRead, LinesCodec};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Rust Compiler API is running!" }))
        .route("/compile", post(compile_and_run))
        .route("/ws", get(ws_handler))
        .layer(cors);

    // Run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // 1. Wait for the first message which should be the code
    let code = if let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            // Try to parse as JSON first, or just take raw text if simple
            if let Ok(req) = serde_json::from_str::<CodeRequest>(&text) {
                req.code
            } else {
                // Fallback if client sends just the code string
                text
            }
        } else {
            return;
        }
    } else {
        return;
    };

    let id = Uuid::new_v4();
    let _ = fs::create_dir_all("temp").await;
    
    let filename = format!("temp/temp_{}.rs", id);
    let exe_name = if cfg!(target_os = "windows") {
        format!("temp/temp_{}.exe", id)
    } else {
        format!("temp/temp_{}", id)
    };
    
    let run_path = if cfg!(target_os = "windows") {
        format!(".\\temp\\temp_{}.exe", id)
    } else {
        format!("./temp/temp_{}", id)
    };

    // Write code to file
    if let Err(e) = fs::write(&filename, &code).await {
        let _ = socket.send(Message::Text(format!("Failed to write file: {}", e))).await;
        return;
    }

    // Compile
    let compile_output = Command::new("rustc")
        .arg(&filename)
        .arg("-o")
        .arg(&exe_name)
        .output()
        .await;

    match compile_output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let _ = socket.send(Message::Text(stderr)).await;
                let _ = fs::remove_file(&filename).await;
                return;
            }
        }
        Err(e) => {
            let _ = socket.send(Message::Text(format!("Failed to execute rustc: {}", e))).await;
            let _ = fs::remove_file(&filename).await;
            return;
        }
    }

    // Run the executable
    let mut child = Command::new(&run_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match child {
        Ok(mut child) => {
            let mut stdin = child.stdin.take().expect("failed to get stdin");
            let stdout = child.stdout.take().expect("failed to get stdout");
            let stderr = child.stderr.take().expect("failed to get stderr");

            let (mut sender, mut receiver) = socket.split();

            // Task to handle stdout/stderr -> WebSocket
            let mut stdout_reader = BufReader::new(stdout);
            let mut stderr_reader = BufReader::new(stderr);

            let mut output_task = tokio::spawn(async move {
                let mut stdout_buf = [0u8; 1024];
                let mut stderr_buf = [0u8; 1024];

                loop {
                    tokio::select! {
                        result = stdout_reader.read(&mut stdout_buf) => {
                            match result {
                                Ok(0) => break, // EOF
                                Ok(n) => {
                                    let text = String::from_utf8_lossy(&stdout_buf[..n]).to_string();
                                    if sender.send(Message::Text(text)).await.is_err() {
                                        break;
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                        result = stderr_reader.read(&mut stderr_buf) => {
                            match result {
                                Ok(0) => break, // EOF
                                Ok(n) => {
                                    let text = String::from_utf8_lossy(&stderr_buf[..n]).to_string();
                                    if sender.send(Message::Text(text)).await.is_err() {
                                        break;
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                    }
                }
            });

            // Task to handle WebSocket -> stdin
            let mut input_task = tokio::spawn(async move {
                while let Some(Ok(msg)) = receiver.next().await {
                    if let Message::Text(text) = msg {
                        // Append newline if missing, as read_line usually expects it
                        let input = if text.ends_with('\n') { text } else { text + "\n" };
                        if stdin.write_all(input.as_bytes()).await.is_err() {
                            break;
                        }
                        if stdin.flush().await.is_err() {
                            break;
                        }
                    } else if let Message::Close(_) = msg {
                        break;
                    }
                }
            });

            // Wait for child to finish or tasks to end
            tokio::select! {
                _ = child.wait() => {},
                _ = &mut output_task => {},
                _ = &mut input_task => {},
            }

            // Cleanup
            let _ = fs::remove_file(&filename).await;
            let _ = fs::remove_file(&exe_name).await;
            if cfg!(target_os = "windows") {
                let pdb_name = format!("temp/temp_{}.pdb", id);
                let _ = fs::remove_file(&pdb_name).await;
            }
        }
        Err(e) => {
            let _ = socket.send(Message::Text(format!("Failed to spawn process: {}", e))).await;
            let _ = fs::remove_file(&filename).await;
            let _ = fs::remove_file(&exe_name).await;
        }
    }
}

#[derive(Deserialize)]
struct CodeRequest {
    code: String,
}

async fn compile_and_run(
    Json(payload): Json<CodeRequest>,
) -> Sse<std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {
    let code = payload.code;
    let id = Uuid::new_v4();
    let _ = fs::create_dir_all("temp").await;
    
    let filename = format!("temp/temp_{}.rs", id);
    let exe_name = if cfg!(target_os = "windows") {
        format!("temp/temp_{}.exe", id)
    } else {
        format!("temp/temp_{}", id)
    };
    
    // For running, we need the path relative to current dir or absolute.
    let run_path = if cfg!(target_os = "windows") {
        format!(".\\temp\\temp_{}.exe", id)
    } else {
        format!("./temp/temp_{}", id)
    };

    // Write code to file
    if let Err(e) = fs::write(&filename, &code).await {
        let stream = stream::once(async move {
            Ok(Event::default().data(format!("Failed to write file: {}", e)))
        });
        return Sse::new(Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>).keep_alive(KeepAlive::default());
    }

    // Compile
    let compile_output = Command::new("rustc")
        .arg(&filename)
        .arg("-o")
        .arg(&exe_name)
        .output()
        .await;

    match compile_output {
        Ok(output) => {
            if !output.status.success() {
                // Compilation failed
                let _ = fs::remove_file(&filename).await;
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let stream = stream::once(async move {
                    Ok(Event::default().data(stderr))
                });
                return Sse::new(Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>).keep_alive(KeepAlive::default());
            }
        }
        Err(e) => {
            let _ = fs::remove_file(&filename).await;
            let stream = stream::once(async move {
                Ok(Event::default().data(format!("Failed to execute rustc: {}", e)))
            });
            return Sse::new(Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>).keep_alive(KeepAlive::default());
        }
    }

    // Run the executable
    let mut child = Command::new(&run_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match child {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("child did not have a handle to stdout");
            let stderr = child.stderr.take().expect("child did not have a handle to stderr");

            let stdout_stream = FramedRead::new(stdout, LinesCodec::new())
                .map(|line| line.map_err(|e| e.to_string()));
            let stderr_stream = FramedRead::new(stderr, LinesCodec::new())
                .map(|line| line.map_err(|e| e.to_string()));

            let merged_stream = stream::select(stdout_stream, stderr_stream)
                .map(|result| {
                    match result {
                        Ok(line) => Ok(Event::default().data(line)),
                        Err(e) => Ok(Event::default().data(format!("Error reading output: {}", e))),
                    }
                });

            // Cleanup task
            tokio::spawn(async move {
                let _ = child.wait().await;
                let _ = fs::remove_file(&filename).await;
                let _ = fs::remove_file(&exe_name).await;
                if cfg!(target_os = "windows") {
                    let pdb_name = format!("temp/temp_{}.pdb", id);
                    let _ = fs::remove_file(&pdb_name).await;
                }
            });

            Sse::new(Box::pin(merged_stream) as std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>).keep_alive(KeepAlive::default())
        }
        Err(e) => {
            let _ = fs::remove_file(&filename).await;
            let _ = fs::remove_file(&exe_name).await;
            if cfg!(target_os = "windows") {
                let pdb_name = format!("temp/temp_{}.pdb", id);
                let _ = fs::remove_file(&pdb_name).await;
            }
            let stream = stream::once(async move {
                Ok(Event::default().data(format!("Failed to spawn process: {}", e)))
            });
            return Sse::new(Box::pin(stream) as std::pin::Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>).keep_alive(KeepAlive::default());
        }
    }
}
