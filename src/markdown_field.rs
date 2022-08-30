use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::{
    log,
    markdown::{Markdown, MarkdownBlock},
};

pub enum MarkdownFieldMessage {
    Editing(Option<usize>),
    Edit,
}

#[derive(Properties, PartialEq)]
pub struct MarkdownFieldProperties {
    #[prop_or_default]
    pub onedit: Callback<Markdown>,
    pub markdown: Markdown,
}

pub struct MarkdownField {
    node_ref: NodeRef,
    editing: Option<usize>,
    selection_start: Option<u32>,
    selection_end: Option<u32>,
}

impl Component for MarkdownField {
    type Message = MarkdownFieldMessage;
    type Properties = MarkdownFieldProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            editing: None,
            selection_start: None,
            selection_end: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MarkdownFieldMessage::Editing(editing) => {
                let element = self.node_ref.cast::<HtmlTextAreaElement>().unwrap();
                self.selection_start = element.selection_start().unwrap();
                self.selection_end = element.selection_end().unwrap();

                self.editing = editing;

                true
            }
            MarkdownFieldMessage::Edit => {
                let element = self.node_ref.cast::<HtmlTextAreaElement>().unwrap();
                let raw_markdown = element.text_content().unwrap();
                let markdown = Markdown::from_str(&raw_markdown);
                self.selection_start = element.selection_start().unwrap();
                self.selection_end = element.selection_end().unwrap();

                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let blocks = ctx
            .props()
            .markdown
            .blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                let editing = self.editing == Some(i);

                html! {
                    <MarkdownBlockField
                        block={ block.clone() }
                        onclick={ ctx.link().callback(move |_| MarkdownFieldMessage::Editing(Some(i))) }
                        {editing}
                    />
                }
            });

        let content = html!({for blocks});

        let last_block = ctx.props().markdown.blocks.len() - 1;
        html! {
            <div class="markdown">
                <div
                    class="markdown-content"
                    contenteditable="true"
                    aria-multiline="true"
                    onkeydown={ ctx.link().callback(|_| MarkdownFieldMessage::Edit) }
                    onclick={ ctx.link().callback(move |_| MarkdownFieldMessage::Editing(Some(last_block))) }
                    onfocusout={ ctx.link().callback(|_| MarkdownFieldMessage::Editing(None)) }
                    ref={ self.node_ref.clone() }
                >
                    { content }
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let element = self.node_ref.cast::<HtmlTextAreaElement>().unwrap();
        element.set_selection_start(self.selection_start).unwrap();
        element.set_selection_end(self.selection_end).unwrap();
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownBlockFieldProperties {
    #[prop_or_default]
    pub editing: bool,
    #[prop_or_default]
    pub block: MarkdownBlock,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub struct MarkdownBlockField {
    node_ref: NodeRef,
}

impl Component for MarkdownBlockField {
    type Message = ();
    type Properties = MarkdownBlockFieldProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let contents = if ctx.props().block.text().is_empty() {
            html!(<br/>)
        } else {
            html! {
                { ctx.props().block.text() }
            }
        };

        html! {
            <div
                class="markdown-block"
                ref={ self.node_ref.clone() }
            >
                { contents }
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if !ctx.props().editing {
            let element = self.node_ref.cast::<HtmlElement>().unwrap();
            element.set_inner_html(&ctx.props().block.html());
        }
    }
}
