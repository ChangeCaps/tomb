use web_sys::HtmlElement;
use yew::prelude::*;

use crate::markdown::{Markdown, MarkdownBlock, MarkdownBlockEdit};

pub enum MarkdownEditorMessage {}

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProperties {
    #[prop_or_default]
    pub markdown: Markdown,
    #[prop_or_default]
    pub onblockedit: Callback<MarkdownBlockEdit>,
}

pub struct MarkdownEditor {
    pub node_refs: Vec<NodeRef>,
}

impl Component for MarkdownEditor {
    type Message = MarkdownEditorMessage;
    type Properties = MarkdownEditorProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let len_blocks = ctx.props().markdown.blocks.len();

        Self {
            node_refs: vec![NodeRef::default(); len_blocks],
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let len_blocks = ctx.props().markdown.blocks.len();

        if self.node_refs.len() != len_blocks {
            self.node_refs = vec![NodeRef::default(); len_blocks];

            true
        } else {
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let blocks = ctx.props().markdown.iter().enumerate().map(|(i, block)| {
            html! {
                <MarkdownEditorBlock
                    block={ block.clone() }
                    node_ref={ self.node_refs[i].clone() }
                    onedit={ ctx.props().onblockedit.reform(move |block| MarkdownBlockEdit(i, block)) }
                />
            }
        });

        let last_node = self.node_refs.last().unwrap().clone();

        let onclick = move |_| {
            let element = last_node.cast::<HtmlElement>().unwrap();
            element.focus().unwrap();
        };

        html! {
            <div class="markdown-view">
                <div
                    class="markdown-content"
                    {onclick}
                >
                    { for blocks }
                </div>
            </div>
        }
    }
}

pub enum MarkdownEditorBlockMessage {
    Focus(bool),
    Edit(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorBlockProperties {
    #[prop_or_default]
    pub block: MarkdownBlock,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub onedit: Callback<MarkdownBlock>,
}

pub struct MarkdownEditorBlock {
    markdown: String,
    edited: bool,
}

impl Component for MarkdownEditorBlock {
    type Message = MarkdownEditorBlockMessage;
    type Properties = MarkdownEditorBlockProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            markdown: String::new(),
            edited: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MarkdownEditorBlockMessage::Focus(focus) => {
                let element = ctx.props().node_ref.cast::<HtmlElement>().unwrap();

                if focus {
                    element.set_inner_text(&self.markdown);
                } else {
                    self.markdown = element.inner_text();
                    let block = MarkdownBlock::new(self.markdown.clone());

                    element.set_inner_html(&block.html());
                    ctx.props().onedit.emit(block);
                }

                self.edited = focus;

                true
            }
            MarkdownEditorBlockMessage::Edit(_event) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = if self.edited {
            "markdown-editor-block"
        } else {
            "markdown-view-block"
        };

        html! {
            <div
                {class}
                contenteditable="true"
                onfocusin={ ctx.link().callback(|_| MarkdownEditorBlockMessage::Focus(true)) }
                onfocusout={ ctx.link().callback(|_| MarkdownEditorBlockMessage::Focus(false)) }
                onkeypress={ ctx.link().callback(MarkdownEditorBlockMessage::Edit) }
                ref={ ctx.props().node_ref.clone() }
            >
                { ctx.props().block.text() }
            </div>
        }
    }
}
