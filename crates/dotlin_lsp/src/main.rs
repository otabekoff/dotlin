use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::*;
#[derive(Debug)]
struct Backend {
    client: Client,
    documents: tokio::sync::RwLock<std::collections::HashMap<String, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "dotlin-lsp".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), "=".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Dotlin server initialized!")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut documents = self.documents.write().await;
        documents.insert(params.text_document.uri.to_string(), params.text_document.text.clone());
        
        self.client
            .log_message(MessageType::INFO, format!("Document opened: {}", params.text_document.uri))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut documents = self.documents.write().await;
        if let Some(change) = params.content_changes.first() {
            documents.insert(params.text_document.uri.to_string(), change.text.clone());
        }
        
        self.client
            .log_message(MessageType::INFO, format!("Document changed: {}", params.text_document.uri))
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut documents = self.documents.write().await;
        documents.remove(&params.text_document.uri.to_string());
        
        self.client
            .log_message(MessageType::INFO, format!("Document closed: {}", params.text_document.uri))
            .await;
    }

    async fn hover(&self, _params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        // For now, return a simple hover response
        let contents = HoverContents::Scalar(MarkedString::String("Dotlin Language Server".to_string()));
        let hover = Hover {
            contents,
            range: None,
        };
        
        Ok(Some(hover))
    }

    async fn completion(&self, _params: CompletionParams) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        // For now, return some basic completions
        let completions = vec![
            CompletionItem {
                label: "fun".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Function definition".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "var".to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some("Variable declaration".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "println".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Print function".to_string()),
                ..Default::default()
            },
        ];
        
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn goto_definition(&self, _params: GotoDefinitionParams) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        // For now, return None
        Ok(None)
    }

    async fn formatting(&self, _params: DocumentFormattingParams) -> tower_lsp::jsonrpc::Result<Option<Vec<TextEdit>>> {
        // For now, return None (no formatting implemented yet)
        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: tokio::sync::RwLock::new(std::collections::HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}