use core::panic;

use crate::json_structures::PerlDocument;
use crate::snippets::get_snippets;
use dashmap::DashMap;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub mod call_parse_perl;
pub mod json_structures;
pub mod snippets;

#[derive(Debug)]
struct Backend {
    client: Client,
    documents_map: DashMap<Url, String>,
    ppi_map: DashMap<Url, PerlDocument>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                    // TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["->".to_string(), "::".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                definition_provider: Some(OneOf::Right(DefinitionOptions {
                    work_done_progress_options: Default::default(),
                })),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::ERROR,
                format!("file opened: {:?}", params.text_document.uri.to_string()),
            )
            .await;

        let parsed_document = call_parse_perl::run_parse_perl(params.text_document.text);

        if parsed_document.is_err() {
            self.client
                .log_message(MessageType::ERROR, parsed_document.as_ref().unwrap())
                .await;
            return;
        }

        let json_document = json_structures::parse_json(&parsed_document.unwrap());
        if json_document.is_err() {
            self.client
                .log_message(MessageType::ERROR, json_document.as_ref().unwrap_err())
                .await;
            return;
        }
        self.ppi_map
            .insert(params.text_document.uri, json_document.unwrap());
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let changes = params.content_changes;
        if changes.len() != 1 {
            panic!("You can only change one document at a time! {:?}", changes);
        }
        let change_text = changes[0].text.clone();
        self.documents_map
            .insert(params.text_document.uri, change_text);

        self.client
            .log_message(
                MessageType::ERROR,
                format!("file changed! {:?}", self.documents_map),
            )
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        get_snippets()
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        self.client
            .log_message(
                MessageType::ERROR,
                format!("Go to Definition! {:?} {:?}", params, self.ppi_map),
            )
            .await;
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: params.text_document_position_params.text_document.uri,
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
        })))
    }
}

#[tokio::main]
async fn main() {
    // #[cfg(feature = "runtime-agnostic")]
    // use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    // #[cfg(feature = "runtime-agnostic")]
    // let (stdin, stdout) = (stdin.compat(), stdout.compat_write());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents_map: DashMap::default(),
        ppi_map: DashMap::default(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
