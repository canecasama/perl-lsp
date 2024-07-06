use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::mem::discriminant;

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    line: i32,
    column: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeData {
    content: Option<String>,
    location: Option<Location>,
    children: Option<Vec<PerlNode>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum PerlNode {
    #[serde(rename = "PPI::Statement")]
    Statement(NodeData),
    #[serde(rename = "PPI::Statement::Break")]
    StatementBreak(NodeData),
    #[serde(rename = "PPI::Statement::Compound")]
    StatementCompound(NodeData),
    #[serde(rename = "PPI::Statement::Data")]
    StatementData(NodeData),
    #[serde(rename = "PPI::Statement::End")]
    StatementEnd(NodeData),
    #[serde(rename = "PPI::Statement::Expression")]
    StatementExpression(NodeData),
    #[serde(rename = "PPI::Statement::Given")]
    StatementGiven(NodeData),
    #[serde(rename = "PPI::Statement::Include")]
    StatementInclude(NodeData),
    #[serde(rename = "PPI::Statement::Include::Perl6")]
    StatementIncludePerlSix(NodeData),
    #[serde(rename = "PPI::Statement::Null")]
    StatementNull(NodeData),
    #[serde(rename = "PPI::Statement::Package")]
    StatementPackage(NodeData),
    #[serde(rename = "PPI::Statement::Scheduled")]
    StatementScheduled(NodeData),
    #[serde(rename = "PPI::Statement::Sub")]
    StatementSub(NodeData),
    #[serde(rename = "PPI::Statement::Unknown")]
    StatementUnknown(NodeData),
    #[serde(rename = "PPI::Statement::UnmatchedBrace")]
    StatementUnmatchedBrace(NodeData),
    #[serde(rename = "PPI::Statement::Variable")]
    StatementVariable(NodeData),
    #[serde(rename = "PPI::Statement::When")]
    StatementWhen(NodeData),
    #[serde(rename = "PPI::Structure")]
    Structure(NodeData),
    #[serde(rename = "PPI::Structure::Block")]
    StructureBlock(NodeData),
    #[serde(rename = "PPI::Structure::Condition")]
    StructureCondition(NodeData),
    #[serde(rename = "PPI::Structure::Constructor")]
    StructureConstructor(NodeData),
    #[serde(rename = "PPI::Structure::For")]
    StructureFor(NodeData),
    #[serde(rename = "PPI::Structure::Given")]
    StructureGiven(NodeData),
    #[serde(rename = "PPI::Structure::List")]
    StructureList(NodeData),
    #[serde(rename = "PPI::Structure::Subscript")]
    StructureSubscript(NodeData),
    #[serde(rename = "PPI::Structure::Unknown")]
    StructureUnknown(NodeData),
    #[serde(rename = "PPI::Structure::When")]
    StructureWhen(NodeData),
    #[serde(rename = "PPI::Token")]
    Token(NodeData),
    #[serde(rename = "PPI::Token::ArrayIndex")]
    TokenArrayIndex(NodeData),
    #[serde(rename = "PPI::Token::Attribute")]
    TokenAttribute(NodeData),
    #[serde(rename = "PPI::Token::BOM")]
    TokenBOM(NodeData),
    #[serde(rename = "PPI::Token::Cast")]
    TokenCast(NodeData),
    #[serde(rename = "PPI::Token::Comment")]
    TokenComment(NodeData),
    #[serde(rename = "PPI::Token::DashedWord")]
    TokenDashedWord(NodeData),
    #[serde(rename = "PPI::Token::Data")]
    TokenData(NodeData),
    #[serde(rename = "PPI::Token::End")]
    TokenEnd(NodeData),
    #[serde(rename = "PPI::Token::HereDoc")]
    TokenHereDoc(NodeData),
    #[serde(rename = "PPI::Token::Label")]
    TokenLabel(NodeData),
    #[serde(rename = "PPI::Token::Magic")]
    TokenMagic(NodeData),
    #[serde(rename = "PPI::Token::Number")]
    TokenNumber(NodeData),
    #[serde(rename = "PPI::Token::Number::Binary")]
    TokenNumberBinary(NodeData),
    #[serde(rename = "PPI::Token::Number::Exp")]
    TokenNumberExp(NodeData),
    #[serde(rename = "PPI::Token::Number::Float")]
    TokenNumberFloat(NodeData),
    #[serde(rename = "PPI::Token::Number::Hex")]
    TokenNumberHex(NodeData),
    #[serde(rename = "PPI::Token::Number::Octal")]
    TokenNumberOctal(NodeData),
    #[serde(rename = "PPI::Token::Number::Version")]
    TokenNumberVersion(NodeData),
    #[serde(rename = "PPI::Token::Operator")]
    TokenOperator(NodeData),
    #[serde(rename = "PPI::Token::Pod")]
    TokenPod(NodeData),
    #[serde(rename = "PPI::Token::Prototype")]
    TokenPrototype(NodeData),
    #[serde(rename = "PPI::Token::Quote")]
    TokenQuote(NodeData),
    #[serde(rename = "PPI::Token::Quote::Double")]
    TokenQuoteDouble(NodeData),
    #[serde(rename = "PPI::Token::Quote::Interpolate")]
    TokenQuoteInterpolate(NodeData),
    #[serde(rename = "PPI::Token::Quote::Literal")]
    TokenQuoteLiteral(NodeData),
    #[serde(rename = "PPI::Token::Quote::Single")]
    TokenQuoteSingle(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike")]
    TokenQuoteLike(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike::Backtick")]
    TokenQuoteLikeBacktick(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike::Command")]
    TokenQuoteLikeCommand(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike::Readline")]
    TokenQuoteLikeReadline(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike::Regexp")]
    TokenQuoteLikeRegexp(NodeData),
    #[serde(rename = "PPI::Token::QuoteLike::Words")]
    TokenQuoteLikeWord(NodeData),
    #[serde(rename = "PPI::Token::Regexp")]
    TokenRegexp(NodeData),
    #[serde(rename = "PPI::Token::Regexp::Match")]
    TokenRegexpMatch(NodeData),
    #[serde(rename = "PPI::Token::Regexp::Substitute")]
    TokenRegexpSubstitute(NodeData),
    #[serde(rename = "PPI::Token::Regexp::Transliterate")]
    TokenRegexpTransliterate(NodeData),
    #[serde(rename = "PPI::Token::Separator")]
    TokenSeparator(NodeData),
    #[serde(rename = "PPI::Token::Structure")]
    TokenStructure(NodeData),
    #[serde(rename = "PPI::Token::Symbol")]
    TokenSymbol(NodeData),
    #[serde(rename = "PPI::Token::Unknown")]
    TokenUnknown(NodeData),
    #[serde(rename = "PPI::Token::Whitespace")]
    TokenWhitespace(NodeData),
    #[serde(rename = "PPI::Token::Word")]
    TokenWord(NodeData),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerlDocument {
    content: Option<String>,
    location: Option<Location>,
    children: Option<Vec<PerlNode>>,
}

trait FindNode {
    fn find_node_by_type(&self, node_type: &PerlNode) -> Option<&PerlNode>;
}

impl FindNode for PerlDocument {
    fn find_node_by_type(&self, node_type: &PerlNode) -> Option<&PerlNode> {
        if let Some(children) = &self.children {
            for child in children {
                if let Some(node) = child.find_node_by_type(node_type) {
                    return Some(node);
                }
            }
        }
        None
    }
}

impl FindNode for PerlNode {
    fn find_node_by_type(&self, node_type: &PerlNode) -> Option<&PerlNode> {
        if discriminant(self) == discriminant(node_type) {
            return Some(self);
            // } else if let Some(children) = &self.children {
            //     for child in children {
            //         if let Some(node) = child.find_node_by_type(node_type) {
            //             return Some(node);
            //         }
            //     }
        }
        None
    }
}

pub fn parse_json(input: &str) -> Result<PerlDocument, Error> {
    serde_json::from_str(input)
}
