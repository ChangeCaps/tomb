mod invoke;
mod log;
mod markdown;
mod markdown_field;

use crate::markdown::Markdown;
use yew::prelude::*;

use markdown_field::MarkdownField;

enum ModelMessage {
    MarkdownEdit(Markdown),
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
            ModelMessage::MarkdownEdit(new_markdown) => {
                self.markdown = new_markdown;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                <MarkdownField
                    onedit={ ctx.link().callback(ModelMessage::MarkdownEdit) }
                    markdown={ self.markdown.clone() }
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
