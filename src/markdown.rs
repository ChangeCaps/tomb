use pulldown_cmark::{html, Options, Parser};

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

    pub fn iter(&self) -> impl Iterator<Item = &MarkdownBlock> {
        self.blocks.iter()
    }

    pub fn block_edit(&mut self, MarkdownBlockEdit(i, block): MarkdownBlockEdit) {
        self.blocks[i] = block;
    }

    pub fn block_insert(&mut self, MarkdownBlockInsert(i, block): MarkdownBlockInsert) {
        self.blocks.insert(i, block);
    }

    pub fn block_remove(&mut self, MarkdownBlockRemove(i): MarkdownBlockRemove) {
        self.blocks.remove(i);
    }
}

pub struct MarkdownBlockEdit(pub usize, pub MarkdownBlock);
pub struct MarkdownBlockInsert(pub usize, pub MarkdownBlock);
pub struct MarkdownBlockRemove(pub usize);

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
        let parser = Parser::new_ext(&self.text, Options::all());

        let mut html = String::new();
        html::push_html(&mut html, parser);

        html
    }
}
