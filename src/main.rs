use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let my_json = r#"
    {
        "article": "How to work with Json in Rust",
        "author": "Muktar Sadiq",
        "paragraph": [
            {
            "name": "starting sentence"
            },
            {
            "name": "body of the pragraph"
            },
            {
            "name": "end of the paragraph"
            }
        ]
    }"#;

    let parse: Article = read_json_type(my_json);
    println!(
        "\n\nThe name of the first paragraph is : {}",
        parse.paragraph[0].name
    );
}

fn read_json_type(raw_Json: &str) -> Article {
    let parse: Article = serde_json::from_str(raw_Json).unwrap();
    return parse;
}
