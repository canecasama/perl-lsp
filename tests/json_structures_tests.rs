use perl_lsp_rust::json_structures::{Location, NodeData, PerlDocument, PerlNode};

#[test]
fn test_search_by_type_and_content() {
    let json = r#"{
        "children": [
            {
                "location": {"line": 1, "column": 1},
                "type": "PPI::Statement::Variable",
                "children": [
                    {"content": "my", "location": {"line": 1, "column": 1}, "type": "PPI::Token::Word"},
                    {"content": "=", "type": "PPI::Token::Operator", "location": {"line": 1, "column": 5}},
                    {"location": {"line": 1, "column": 7}, "type": "PPI::Token::Number", "content": "1"},
                    {"location": {"line": 1, "column": 8}, "type": "PPI::Token::Structure", "content": ";"}
                ]
            },
            {
                "location": {"line": 2, "column": 1},
                "type": "PPI::Statement::Variable",
                "children": [
                    {"content": "my", "location": {"line": 2, "column": 1}, "type": "PPI::Token::Word"},
                    {"content": "=", "type": "PPI::Token::Operator", "location": {"line": 2, "column": 5}},
                    {"location": {"line": 2, "column": 7}, "type": "PPI::Token::Number", "content": "2"},
                    {"location": {"line": 2, "column": 8}, "type": "PPI::Token::Structure", "content": ";"}
                ]
            }
        ],
        "type": "PPI::Document",
        "location": {"line": 1, "column": 1}
    }"#;

    let tree: PerlDocument = serde_json::from_str(json).unwrap();

    // Asserting the full tree
    assert_eq!(
        tree,
        PerlDocument(NodeData {
            node: PerlNode::Document,
            content: None,
            location: Some(Location { line: 1, column: 1 }),
            children: Some(vec![
                NodeData {
                    node: PerlNode::StatementVariable,
                    content: None,
                    location: Some(Location { line: 1, column: 1 }),
                    children: Some(vec![
                        NodeData {
                            node: PerlNode::TokenWord,
                            content: Some("my".to_string()),
                            location: Some(Location { line: 1, column: 1 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenOperator,
                            content: Some("=".to_string()),
                            location: Some(Location { line: 1, column: 5 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenNumber,
                            content: Some("1".to_string()),
                            location: Some(Location { line: 1, column: 7 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenStructure,
                            content: Some(";".to_string()),
                            location: Some(Location { line: 1, column: 8 }),
                            children: None,
                        },
                    ]),
                },
                NodeData {
                    node: PerlNode::StatementVariable,
                    content: None,
                    location: Some(Location { line: 2, column: 1 }),
                    children: Some(vec![
                        NodeData {
                            node: PerlNode::TokenWord,
                            content: Some("my".to_string()),
                            location: Some(Location { line: 2, column: 1 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenOperator,
                            content: Some("=".to_string()),
                            location: Some(Location { line: 2, column: 5 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenNumber,
                            content: Some("2".to_string()),
                            location: Some(Location { line: 2, column: 7 }),
                            children: None,
                        },
                        NodeData {
                            node: PerlNode::TokenStructure,
                            content: Some(";".to_string()),
                            location: Some(Location { line: 2, column: 8 }),
                            children: None,
                        },
                    ]),
                },
            ]),
        })
    );

    // Searching for nodes of type PPI::Token::Word with content "my"
    let results = tree.0.search_by_type_and_content(PerlNode::TokenWord, "my");
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].content.as_deref(), Some("my"));

    // Searching for nodes of type PPI::Token::Number with content "1"
    let results = tree
        .0
        .search_by_type_and_content(PerlNode::TokenNumber, "1");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content.as_deref(), Some("1"));
}
