use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower_lsp::{async_trait, jsonrpc::Error, Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::{notification::Notification, ExecuteCommandOptions, ExecuteCommandParams, InitializeParams, InitializeResult, MessageType, ServerCapabilities};
use tower_lsp::jsonrpc::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationParams {
    title: String,
    message: String,
    description: String,
}
enum CustomNotification {}

impl Notification for CustomNotification {
    type Params = NotificationParams;
    const METHOD: &'static str = "custom/notification";
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None, //: ServerInfo,
            capabilities: ServerCapabilities {
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["custom.notification".into()],
                    work_done_progress_options: Default::default(),
                }),
                ..ServerCapabilities::default()
            }
    })}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        if params.command == "custom.notification" {
            self.client.send_notification::<CustomNotification>(NotificationParams {
                title: String::from("tit"),
                message: String::from("mes"),
                description: String::from("des"),
            }).await;

            self.client.log_message(MessageType::INFO, format!("Command executed Succesfully with params {params:?}"))
                .await;
            Ok(None)
        } else {
            Err(Error::invalid_request())
        }
    }
}

#[tokio::main]
pub async fn main() {
    println!("Hello, server start.");
    tracing_subscriber::fmt().init();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
    .await.unwrap();

    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);
    let (service, socket) = LspService::new(|client| Backend {
        client
    });

    Server::new(read, write, socket).serve(service).await;

    println!("Hello, server end.");
}