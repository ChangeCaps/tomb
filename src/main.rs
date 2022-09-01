mod document;
mod document_editor;
mod log;

use document::{
    Document, DocumentBlockEdit, DocumentBlockInsert, DocumentBlockMerge, DocumentBlockRemove,
};
use yew::prelude::*;

use document_editor::DocumentEditor;

enum ModelMessage {
    BlockEdit(DocumentBlockEdit),
    BlockInsert(DocumentBlockInsert),
    BlockRemove(DocumentBlockRemove),
    BlockMerge(DocumentBlockMerge),
}

#[derive(Properties, PartialEq, Default)]
struct ModelProperties {}

struct Model {
    document: Document,
}

impl Component for Model {
    type Message = ModelMessage;
    type Properties = ModelProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            document: Document::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModelMessage::BlockEdit(edit) => {
                self.document.block_edit(edit);

                true
            }
            ModelMessage::BlockInsert(insert) => {
                self.document.block_insert(insert);

                true
            }
            ModelMessage::BlockRemove(remove) => {
                self.document.block_remove(remove);

                true
            }
            ModelMessage::BlockMerge(merge) => {
                self.document.block_merge(merge);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                <DocumentEditor
                    document={ self.document.clone() }
                    onblockedit={ ctx.link().callback(ModelMessage::BlockEdit) }
                    onblockinsert={ ctx.link().callback(ModelMessage::BlockInsert) }
                    onblockmerge={ ctx.link().callback(ModelMessage::BlockMerge) }
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
