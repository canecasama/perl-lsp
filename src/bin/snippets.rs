use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

fn new_snippet_item(label: String, detail: String, snippet: String) -> CompletionItem {
    CompletionItem {
        label,
        detail: Some(detail),
        kind: Some(CompletionItemKind::SNIPPET),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some(snippet),
        ..Default::default()
    }
}

pub fn get_snippets() -> Result<Option<CompletionResponse>> {
    Ok(Some(CompletionResponse::Array(vec![
        new_snippet_item(
            "sub".to_string(),
            "Insert subroutine".to_string(),
            "sub $1\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "foreach".to_string(),
            "Insert foreach loop".to_string(),
            "foreach my $1 ($2)\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "for".to_string(),
            "Insert C-style for loop".to_string(),
            "for ($1 ; $2 ; $3)\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "while".to_string(),
            "Insert while statement".to_string(),
            "while ($1)\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "if".to_string(),
            "Insert if statement".to_string(),
            "if ($1)\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "elsif".to_string(),
            "Insert elsif statement".to_string(),
            "elsif ($1)\n{\n\t$0\n}".to_string(),
        ),
        new_snippet_item(
            "else".to_string(),
            "Insert else statement".to_string(),
            "else\n{\n\t$0\n}".to_string(),
        ),
        // new_snippet_item("".to_string(), "".to_string(), "".to_string()),
    ])))
}
