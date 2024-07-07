use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    line: i32,
    column: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeData {
    #[serde(rename = "type")]
    node_type: PerlNode,
    content: Option<String>,
    location: Option<Location>,
    children: Option<Vec<PerlNode>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum PerlNode {
    #[serde(rename = "PPI::Document")]
    Document,
    #[serde(rename = "PPI::Statement")]
    Statement,
    #[serde(rename = "PPI::Statement::Break")]
    StatementBreak,
    #[serde(rename = "PPI::Statement::Compound")]
    StatementCompound,
    #[serde(rename = "PPI::Statement::Data")]
    StatementData,
    #[serde(rename = "PPI::Statement::End")]
    StatementEnd,
    #[serde(rename = "PPI::Statement::Expression")]
    StatementExpression,
    #[serde(rename = "PPI::Statement::Given")]
    StatementGiven,
    #[serde(rename = "PPI::Statement::Include")]
    StatementInclude,
    #[serde(rename = "PPI::Statement::Include::Perl6")]
    StatementIncludePerlSix,
    #[serde(rename = "PPI::Statement::Null")]
    StatementNull,
    #[serde(rename = "PPI::Statement::Package")]
    StatementPackage,
    #[serde(rename = "PPI::Statement::Scheduled")]
    StatementScheduled,
    #[serde(rename = "PPI::Statement::Sub")]
    StatementSub,
    #[serde(rename = "PPI::Statement::Unknown")]
    StatementUnknown,
    #[serde(rename = "PPI::Statement::UnmatchedBrace")]
    StatementUnmatchedBrace,
    #[serde(rename = "PPI::Statement::Variable")]
    StatementVariable,
    #[serde(rename = "PPI::Statement::When")]
    StatementWhen,
    #[serde(rename = "PPI::Structure")]
    Structure,
    #[serde(rename = "PPI::Structure::Block")]
    StructureBlock,
    #[serde(rename = "PPI::Structure::Condition")]
    StructureCondition,
    #[serde(rename = "PPI::Structure::Constructor")]
    StructureConstructor,
    #[serde(rename = "PPI::Structure::For")]
    StructureFor,
    #[serde(rename = "PPI::Structure::Given")]
    StructureGiven,
    #[serde(rename = "PPI::Structure::List")]
    StructureList,
    #[serde(rename = "PPI::Structure::Subscript")]
    StructureSubscript,
    #[serde(rename = "PPI::Structure::Unknown")]
    StructureUnknown,
    #[serde(rename = "PPI::Structure::When")]
    StructureWhen,
    #[serde(rename = "PPI::Token")]
    Token,
    #[serde(rename = "PPI::Token::ArrayIndex")]
    TokenArrayIndex,
    #[serde(rename = "PPI::Token::Attribute")]
    TokenAttribute,
    #[serde(rename = "PPI::Token::BOM")]
    TokenBOM,
    #[serde(rename = "PPI::Token::Cast")]
    TokenCast,
    #[serde(rename = "PPI::Token::Comment")]
    TokenComment,
    #[serde(rename = "PPI::Token::DashedWord")]
    TokenDashedWord,
    #[serde(rename = "PPI::Token::Data")]
    TokenData,
    #[serde(rename = "PPI::Token::End")]
    TokenEnd,
    #[serde(rename = "PPI::Token::HereDoc")]
    TokenHereDoc,
    #[serde(rename = "PPI::Token::Label")]
    TokenLabel,
    #[serde(rename = "PPI::Token::Magic")]
    TokenMagic,
    #[serde(rename = "PPI::Token::Number")]
    TokenNumber,
    #[serde(rename = "PPI::Token::Number::Binary")]
    TokenNumberBinary,
    #[serde(rename = "PPI::Token::Number::Exp")]
    TokenNumberExp,
    #[serde(rename = "PPI::Token::Number::Float")]
    TokenNumberFloat,
    #[serde(rename = "PPI::Token::Number::Hex")]
    TokenNumberHex,
    #[serde(rename = "PPI::Token::Number::Octal")]
    TokenNumberOctal,
    #[serde(rename = "PPI::Token::Number::Version")]
    TokenNumberVersion,
    #[serde(rename = "PPI::Token::Operator")]
    TokenOperator,
    #[serde(rename = "PPI::Token::Pod")]
    TokenPod,
    #[serde(rename = "PPI::Token::Prototype")]
    TokenPrototype,
    #[serde(rename = "PPI::Token::Quote")]
    TokenQuote,
    #[serde(rename = "PPI::Token::Quote::Double")]
    TokenQuoteDouble,
    #[serde(rename = "PPI::Token::Quote::Interpolate")]
    TokenQuoteInterpolate,
    #[serde(rename = "PPI::Token::Quote::Literal")]
    TokenQuoteLiteral,
    #[serde(rename = "PPI::Token::Quote::Single")]
    TokenQuoteSingle,
    #[serde(rename = "PPI::Token::QuoteLike")]
    TokenQuoteLike,
    #[serde(rename = "PPI::Token::QuoteLike::Backtick")]
    TokenQuoteLikeBacktick,
    #[serde(rename = "PPI::Token::QuoteLike::Command")]
    TokenQuoteLikeCommand,
    #[serde(rename = "PPI::Token::QuoteLike::Readline")]
    TokenQuoteLikeReadline,
    #[serde(rename = "PPI::Token::QuoteLike::Regexp")]
    TokenQuoteLikeRegexp,
    #[serde(rename = "PPI::Token::QuoteLike::Words")]
    TokenQuoteLikeWord,
    #[serde(rename = "PPI::Token::Regexp")]
    TokenRegexp,
    #[serde(rename = "PPI::Token::Regexp::Match")]
    TokenRegexpMatch,
    #[serde(rename = "PPI::Token::Regexp::Substitute")]
    TokenRegexpSubstitute,
    #[serde(rename = "PPI::Token::Regexp::Transliterate")]
    TokenRegexpTransliterate,
    #[serde(rename = "PPI::Token::Separator")]
    TokenSeparator,
    #[serde(rename = "PPI::Token::Structure")]
    TokenStructure,
    #[serde(rename = "PPI::Token::Symbol")]
    TokenSymbol,
    #[serde(rename = "PPI::Token::Unknown")]
    TokenUnknown,
    #[serde(rename = "PPI::Token::Whitespace")]
    TokenWhitespace,
    #[serde(rename = "PPI::Token::Word")]
    TokenWord,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerlDocument(NodeData);

trait Search {
    fn search_content(&self, search_str: &str) -> Vec<&NodeData>;
    fn search_by_type_and_content(&self, node_type: &PerlNode, search_str: &str) -> Vec<&NodeData>;
}

impl Search for NodeData {
    fn search_content(&self, search_str: &str) -> Vec<&NodeData> {
        let mut results = Vec::new();

        if let Some(ref content) = self.content {
            if content == search_str {
                results.push(self);
            }
        }

        if let Some(ref children) = self.children {
            for child in children {
                results.extend(child.search_content(search_str));
            }
        }

        results
    }

    fn search_by_type_and_content(&self, node_type: &PerlNode, search_str: &str) -> Vec<&NodeData> {
        let mut results = Vec::new();

        if self.node_type == *node_type {
            if let Some(ref content) = self.content {
                if content == search_str {
                    results.push(self);
                }
            }
        }

        if let Some(ref children) = self.children {
            for child in children {
                results.extend(child.search_by_type_and_content(&node_type, search_str));
            }
        }

        results
    }
}

pub fn parse_json(input: &str) -> Result<PerlDocument, Error> {
    serde_json::from_str(input)
}

// fn main() {
//     let json = r#"{
//         "children": [
//             {
//                 "location": {"line": 1, "column": 1},
//                 "type": "PPI::Statement::Variable",
//                 "children": [
//                     {"content": "my", "location": {"line": 1, "column": 1}, "type": "PPI::Token::Word"},
//                     {"content": "=", "type": "PPI::Token::Operator", "location": {"line": 1, "column": 5}},
//                     {"location": {"line": 1, "column": 7}, "type": "PPI::Token::Number", "content": "1"},
//                     {"location": {"line": 1, "column": 8}, "type": "PPI::Token::Structure", "content": ";"}
//                 ]
//             },
//             {
//                 "location": {"line": 2, "column": 1},
//                 "type": "PPI::Statement::Variable",
//                 "children": [
//                     {"content": "my", "location": {"line": 2, "column": 1}, "type": "PPI::Token::Word"},
//                     {"content": "=", "type": "PPI::Token::Operator", "location": {"line": 2, "column": 5}},
//                     {"location": {"line": 2, "column": 7}, "type": "PPI::Token::Number", "content": "2"},
//                     {"location": {"line": 2, "column": 8}, "type": "PPI::Token::Structure", "content": ";"}
//                 ]
//             }
//         ],
//         "type": "PPI::Document",
//         "location": {"line": 1, "column": 1}
//     }"#;
//
//     let tree: PerlDocument = serde_json::from_str(json).unwrap();
//
//     println!("Full Tree: {:?}", tree);
//
//     println!("Node: {:?}", tree.0.node);
//
//     // Getting content of the root node (which is None in this case)
//     println!("Content: {:?}", tree.0.get_content());
//
//     // Searching for content "my"
//     let results = tree.0.search_content("my");
//     for node in results {
//         println!("Content my: {:?}", node);
//     }
//
//     // Searching for content "1"
//     let results = tree.0.search_content("1");
//     for node in results {
//         println!("Content 1: {:?}", node);
//     }
//
//     // Searching for nodes of type PPI::Token::Word with content "my"
//     let results = tree.0.search_by_type_and_content(&PerlNode::TokenWord, "my");
//     for node in results {
//         println!("TokenWorld my: {:?}", node);
//     }
//
//     // Searching for nodes of type PPI::Token::Number with content "1"
//     let results = tree.0.search_by_type_and_content(&PerlNode::TokenNumber, "1");
//     for node in results {
//         println!("TokenNumber 1: {:?}", node);
//     }
// }
