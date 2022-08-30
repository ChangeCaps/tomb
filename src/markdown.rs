#[derive(Clone, Debug, PartialEq)]
pub struct Markdown {
    pub blocks: Vec<MarkdownBlock>,
}

impl Default for Markdown {
    fn default() -> Self {
        Self {
            blocks: vec![MarkdownBlock::default()],
        }
    }
}

impl Markdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_str(string: &str) -> Self {
        let blocks = string
            .split("\n")
            .map(|line| MarkdownBlock::new(line.into()))
            .collect();

        Self { blocks }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarkdownBlock {
    pub text: String,
}

impl MarkdownBlock {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn html(&self) -> String {
        markdown::to_html(&self.text)
    }
}