use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    line: i32,
    column: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum PerlNode {
    #[serde(rename = "PPI::Statement::Include")]
    StatementInclude {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Word")]
    TokenWord {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Structure")]
    TokenStructure {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike::Words")]
    TokenQuoteLikeWord {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Variable")]
    StatementVariable {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Symbol")]
    TokenSymbol {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Operator")]
    TokenOperator {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Sub")]
    StatementSub {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Unknown")]
    StatementUnknown {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::UnmatchedBrace")]
    StatementUnmatchedBrace {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::When")]
    StatementWhen {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure")]
    Structure {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Block")]
    StructureBlock {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Condition")]
    StructureCondition {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Constructor")]
    StructureConstructor {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::For")]
    StructureFor {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Given")]
    StructureGiven {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::List")]
    StructureList {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Subscript")]
    StructureSubscript {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::Unknown")]
    StructureUnknown {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Structure::When")]
    StructureWhen {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token")]
    Token {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::ArrayIndex")]
    TokenArrayIndex {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Attribute")]
    TokenAttribute {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::BOM")]
    TokenBOM {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Cast")]
    TokenCast {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Comment")]
    TokenComment {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::DashedWord")]
    TokenDashedWord {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Data")]
    TokenData {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::End")]
    TokenEnd {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::HereDoc")]
    TokenHereDoc {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Label")]
    TokenLabel {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Magic")]
    TokenMagic {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number")]
    TokenNumber {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Binary")]
    TokenNumberBinary {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Exp")]
    TokenNumberExp {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Float")]
    TokenNumberFloat {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Hex")]
    TokenNumberHex {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Octal")]
    TokenNumberOctal {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Number::Version")]
    TokenNumberVersion {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Pod")]
    TokenPod {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Prototype")]
    TokenPrototype {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Quote")]
    TokenQuote {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Quote::Double")]
    TokenQuoteDouble {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Quote::Interpolate")]
    TokenQuoteInterpolate {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Quote::Literal")]
    TokenQuoteLiteral {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Quote::Single")]
    TokenQuoteSingle {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike")]
    TokenQuoteLike {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike::Backtick")]
    TokenQuoteLikeBacktick {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike::Command")]
    TokenQuoteLikeCommand {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike::Readline")]
    TokenQuoteLikeReadline {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::QuoteLike::Regexp")]
    TokenQuoteLikeRegexp {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Regexp")]
    TokenRegexp {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Regexp::Match")]
    TokenRegexpMatch {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Regexp::Substitute")]
    TokenRegexpSubstitute {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Regexp::Transliterate")]
    TokenRegexpTransliterate {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Separator")]
    TokenSeparator {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Unknown")]
    TokenUnknown {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Token::Whitespace")]
    TokenWhitespace {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement")]
    Statement {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Break")]
    StatementBreak {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Compound")]
    StatementCompound {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Data")]
    StatementData {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::End")]
    StatementEnd {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Expression")]
    StatementExpression {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Given")]
    StatementGiven {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Include::Perl6")]
    StatementIncludePerlSix {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Null")]
    StatementNull {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Package")]
    StatementPackage {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
    #[serde(rename = "PPI::Statement::Scheduled")]
    StatementScheduled {
        content: Option<String>,
        location: Option<Location>,
        children: Option<Vec<PerlNode>>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerlDocument {
    content: Option<String>,
    location: Option<Location>,
    children: Option<Vec<PerlNode>>,
}

pub fn parse_json(input: &str) -> Result<PerlDocument, serde_json::Error> {
    serde_json::from_str(input)
}
