mod invoke;
mod log;
mod markdown;
mod markdown_editor;

use markdown::{Markdown, MarkdownBlockEdit, MarkdownBlockInsert, MarkdownBlockRemove};
use yew::prelude::*;

use markdown_editor::MarkdownEditor;

enum ModelMessage {
    BlockEdit(MarkdownBlockEdit),
    BlockInsert(MarkdownBlockInsert),
    BlockRemove(MarkdownBlockRemove),
}

#[derive(Properties, PartialEq, Default)]
struct ModelProperties {}

struct Model {
    markdown: Markdown,
}

impl Component for Model {
    type Message = ModelMessage;
    type Properties = ModelProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            markdown: Markdown::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModelMessage::BlockEdit(edit) => {
                self.markdown.block_edit(edit);

                true
            }
            ModelMessage::BlockInsert(insert) => {
                self.markdown.block_insert(insert);
                log!("block insert");

                true
            }
            ModelMessage::BlockRemove(remove) => {
                self.markdown.block_remove(remove);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                <MarkdownEditor
                    markdown={ self.markdown.clone() }
                    onblockedit={ ctx.link().callback(ModelMessage::BlockEdit) }
                    onblockinsert={ ctx.link().callback(ModelMessage::BlockInsert) }
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
