use std::error::Error;
use tokio::task::JoinHandle;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;


async fn run_client() -> Result<(), Box<dyn Error>> {
    println!("Hello, client start");

    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    let initialize_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processid": null,
            "rootUri": null,
            "capabilities": {},
        }
    });
    let initialize_request_str = initialize_request.to_string();
    let initialize_request_formattet = format!(
        "Content-Length: {}\r\n\r\n{}", initialize_request_str.len(), initialize_request_str);
    stream.write_all(initialize_request_formattet.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("received initialize response {}", response);
    
    
    let execute_command_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "workspace/executeCommand",
        "params": {
            "command": "custom.notification",
            "arguments": [{
                "title": "tit-client",
                "message": "mes-client",
                "description": "des-client",
            }],
            "capabilities": {},
        }
    });
    
    let execute_command_str = execute_command_request.to_string();
    let execute_command_formattet = format!(
        "Content-Length: {}\r\n\r\n{}", execute_command_str.len(), execute_command_str);
    stream.write_all(execute_command_formattet.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("received execute_command response {}", response);
    
    println!("Hello, client end");
    Ok(())
}

fn _run_server() -> JoinHandle<()> {
    tokio::spawn(async {
        let command = "cargo run --bin server --quiet"; // Replace with your desired command

        // Execute the CMD command
        let output = Command::new("cmd")
            .args(["/C", command]) // `/C` runs the command and then terminates
            .output()
            .await;

        // Handle the result of the command
        match output {
            Ok(output) => {
                // Print the output
                println!("Command executed successfully:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(err) => {
                println!("Failed to execute command: {}", err);
            }
        }
    })
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // _run_server();
    run_client().await?;    
    Ok(())
}
