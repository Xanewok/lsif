#[macro_use]
extern crate serde_derive;

extern crate languageserver_types as lsp;

pub use lsp::Url;
pub use lsp::{NumberOrString, Range, Position};

pub type RangeId = lsp::NumberOrString;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationOrRangeId {
    Location(lsp::Location),
    RangeId(RangeId)
}

macro_rules! result_of {
    ($x: tt) => {
        <lsp::lsp_request!($x) as lsp::request::Request>::Result
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: lsp::NumberOrString,
    #[serde(flatten)]
    data: Element
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "label")]
pub enum Vertex {
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
    Project(Project),
    Document(Document),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#ranges
    Range(lsp::Range),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
    ResultSet(ResultSet),

    // Method results
    DefinitionResult { result: DefinitionResultType },
    // TODO: Fix ones below to use the { result: LSIFType } format
    HoverResult(result_of!("textDocument/hover")),
    ReferenceResult(result_of!("textDocument/references")),
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    // ImplementationResult(result_of!("textDocument/implementation")),
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    // TypeDefinitionResult(result_of!("textDocument/typeDefinition")),
    FoldingRangeResult(result_of!("textDocument/foldingRange")),
    DocumentLinkResult(result_of!("textDocument/documentLink")),
    DocumentSymbolResult(result_of!("textDocument/documentSymbol")),
    // TODO (these below and more)
    DiagnosticResult,
    ExportResult,
    ExternalImportResult,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "label")]
pub enum Edge {
    Contains(EdgeData),
    RefersTo(EdgeData),
    Item(Item),

    // Methods
    #[serde(rename = "textDocument/definition")]
    Definition(EdgeData),
    #[serde(rename = "textDocument/declaration")]
    Declaration(EdgeData),
    #[serde(rename = "textDocument/hover")]
    Hover(EdgeData),
    #[serde(rename = "textDocument/references")]
    References(EdgeData),
    #[serde(rename = "textDocument/implementation")]
    Implementation(EdgeData),
    #[serde(rename = "textDocument/typeDefinition")]
    TypeDefinition(EdgeData),
    #[serde(rename = "textDocument/foldingRange")]
    FoldingRange(EdgeData),
    #[serde(rename = "textDocument/documentLink")]
    DocumentLink(EdgeData),
    #[serde(rename = "textDocument/documentSymbol")]
    DocumentSymbol(EdgeData),
    #[serde(rename = "textDocument/diagnostic")]
    Diagnostic(EdgeData),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeData {
    in_v: lsp::NumberOrString,
    out_v: lsp::NumberOrString,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionResultType {
    Scalar(LocationOrRangeId),
    Array(LocationOrRangeId),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "property")]
pub enum Item {
    Definition(EdgeData),
    Reference(EdgeData),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(with = "url_serde")]
    uri: lsp::Url,
    language_id: Language
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(with = "url_serde")]
    project_file: lsp::Url,
    language_id: Language
}

/// https://github.com/Microsoft/language-server-protocol/issues/213
/// For examples, see: https://code.visualstudio.com/docs/languages/identifiers.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    TypeScript,
    #[serde(other)]
    Other
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document() {
        let data = Entry {
            id: lsp::NumberOrString::Number(1),
            data: Element::Vertex(Vertex::Document(Document {
                uri: lsp::Url::from_file_path("/Users/dirkb/sample.ts").unwrap(),
                language_id: Language::TypeScript,
            })),
        };

        let text = r#"{ "id": 1, "type": "vertex", "label": "document", "uri": "file:///Users/dirkb/sample.ts", "languageId": "typescript" }"#
            .replace(' ', "");

        assert_eq!(serde_json::to_string(&data).unwrap(), text);
        assert_eq!(serde_json::from_str::<Entry>(&text).unwrap(), data);
    }

    #[test]
    fn range() {
        let data = Entry {
            id: lsp::NumberOrString::Number(4),
            data: Element::Vertex(Vertex::Range(lsp::Range::new(
                lsp::Position::new(0, 9),
                lsp::Position::new(0, 12),
            ))),
        };

        let text = r#"{ "id": 4, "type": "vertex", "label": "range", "start": { "line": 0, "character": 9}, "end": { "line": 0, "character": 12 } }"#
            .replace(' ', "");

        assert_eq!(serde_json::to_string(&data).unwrap(), text);
        assert_eq!(serde_json::from_str::<Entry>(&text).unwrap(), data);
    }

    #[test]
    fn contains() {
        let data = Entry {
            id: lsp::NumberOrString::Number(5),
            data: Element::Edge(Edge::Contains(EdgeData {
                in_v: lsp::NumberOrString::Number(4),
                out_v: lsp::NumberOrString::Number(1),
            })),
        };

        let text = r#"{ "id": 5, "type": "edge", "label": "contains", "outV": 1, "inV": 4}"#
            .replace(' ', "");

        assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), serde_json::to_value(&data).unwrap());
    }

    #[test]
    fn refers_to() {
        let data = Entry {
            id: lsp::NumberOrString::Number(5),
            data: Element::Edge(Edge::RefersTo(EdgeData {
                in_v: lsp::NumberOrString::Number(2),
                out_v: lsp::NumberOrString::Number(3),
            })),
        };

        let text = r#"{ "id": 5, "type": "edge", "label": "refersTo", "outV": 3, "inV": 2}"#
            .replace(' ', "");

        assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), serde_json::to_value(&data).unwrap());
    }

    #[test]
    fn result_set() {
        let data = Entry {
            id: lsp::NumberOrString::Number(2),
            data: Element::Vertex(Vertex::ResultSet(ResultSet { key: None })),
        };

        let text = r#"{ "id": 2, "type": "vertex", "label": "resultSet" }"#
            .replace(' ', "");

        assert_eq!(serde_json::to_string(&data).unwrap(), text);
        assert_eq!(serde_json::from_str::<Entry>(&text).unwrap(), data);

        let data = Entry {
            id: lsp::NumberOrString::Number(4),
            data: Element::Vertex(Vertex::ResultSet(ResultSet { key: Some(String::from("hello")) })),
        };

        let text = r#"{ "id": 4, "type": "vertex", "label": "resultSet", "key": "hello" }"#
            .replace(' ', "");

        assert_eq!(serde_json::to_string(&data).unwrap(), text);
        assert_eq!(serde_json::from_str::<Entry>(&text).unwrap(), data);
    }

    #[test]
    fn definition() {
        let data = Entry {
            id: lsp::NumberOrString::Number(21),
            data: Element::Edge(Edge::Item(Item::Definition(EdgeData {
                in_v: lsp::NumberOrString::Number(18),
                out_v: lsp::NumberOrString::Number(16),
            }))),
        };

        let text = r#"{ "id": 21, "type": "edge", "label": "item", "property": "definition", "outV": 16, "inV": 18}"#
            .replace(' ', "");

        assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), serde_json::to_value(&data).unwrap());
    }

    mod methods {
        use super:: *;

        #[test]
        fn references() {
            let data = Entry {
                id: lsp::NumberOrString::Number(17),
                data: Element::Edge(Edge::References(EdgeData {
                    in_v: lsp::NumberOrString::Number(16),
                    out_v: lsp::NumberOrString::Number(15),
                })),
            };

            let text = r#"{ "id": 17, "type": "edge", "label": "textDocument/references", "outV": 15, "inV": 16 }"#;

            assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), serde_json::to_value(&data).unwrap());
        }

        #[test]
        fn definition() {
            let data = Entry {
                id: lsp::NumberOrString::Number(13),
                data: Element::Vertex(Vertex::DefinitionResult {
                    result: DefinitionResultType::Scalar(LocationOrRangeId::RangeId(lsp::NumberOrString::Number(7))),
                }),
            };

            let text = r#"{ "id": 13, "type": "vertex", "label": "definitionResult", "result": 7 }"#;

            assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(), serde_json::to_value(&data).unwrap());
        }
    }
}
