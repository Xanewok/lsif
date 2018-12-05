#[macro_use]
extern crate serde_derive;

extern crate languageserver_types as lsp;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    id: lsp::NumberOrString,
    #[serde(flatten)]
    data: Element
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "label")]
pub enum Vertex {
    Document(Document),
    Range(lsp::Range)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Edge {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(with = "url_serde")]
    uri: lsp::Url,
    language_id: Language
}

// /// Represents a location inside a resource, such as a line inside a text file.
// #[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
// pub struct Location {
//     #[serde(with = "url_serde")]
//     pub uri: lsp::Url,
//     pub range: lsp::Range,
// }

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
}
