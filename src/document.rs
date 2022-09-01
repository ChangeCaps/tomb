use pulldown_cmark::{html, Options, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub blocks: Vec<DocumentBlock>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            blocks: vec![DocumentBlock::default()],
        }
    }
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DocumentBlock> {
        self.blocks.iter()
    }

    pub fn block_edit(&mut self, DocumentBlockEdit(i, block): DocumentBlockEdit) {
        self.blocks[i] = block;
    }

    pub fn block_insert(&mut self, DocumentBlockInsert(i, block): DocumentBlockInsert) {
        self.blocks.insert(i, block);
    }

    pub fn block_remove(&mut self, DocumentBlockRemove(i): DocumentBlockRemove) {
        self.blocks.remove(i);
    }

    pub fn block_merge(&mut self, DocumentBlockMerge(i, j): DocumentBlockMerge) {
        let removed = self.blocks.remove(j);
        self.blocks[i].text += removed.text();
    }
}

pub struct DocumentBlockEdit(pub usize, pub DocumentBlock);
pub struct DocumentBlockInsert(pub usize, pub DocumentBlock);
pub struct DocumentBlockRemove(pub usize);
pub struct DocumentBlockMerge(pub usize, pub usize);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocumentBlock {
    pub text: String,
}

impl DocumentBlock {
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
