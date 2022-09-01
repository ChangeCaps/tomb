use web_sys::{HtmlElement, window};
use yew::prelude::*;

use crate::document::{Document, DocumentBlock, DocumentBlockEdit, DocumentBlockInsert, DocumentBlockMerge};

pub enum DocumentEditorMessage {
    Insert(DocumentBlockInsert),
    Merge(DocumentBlockMerge),
    StartEdit(usize),
    EndEdit(DocumentBlock),
}

#[derive(Properties, PartialEq)]
pub struct DocumentEditorProperties {
    #[prop_or_default]
    pub document: Document,
    #[prop_or_default]
    pub onblockedit: Callback<DocumentBlockEdit>,
    #[prop_or_default]
    pub onblockinsert: Callback<DocumentBlockInsert>,
    #[prop_or_default]
    pub onblockmerge: Callback<DocumentBlockMerge>,
}

pub struct DocumentEditor {
    node_refs: Vec<NodeRef>,
    editing: Option<usize>,
}

impl Component for DocumentEditor {
    type Message = DocumentEditorMessage;
    type Properties = DocumentEditorProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let len_blocks = ctx.props().document.blocks.len();

        Self {
            node_refs: vec![NodeRef::default(); len_blocks],
            editing: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DocumentEditorMessage::Insert(insert) => {
                self.editing = Some(insert.0);
                ctx.props().onblockinsert.emit(insert);

                true
            },
            DocumentEditorMessage::Merge(merge) => {
                self.editing = Some(merge.0);

                true
            }
            DocumentEditorMessage::StartEdit(i) => {
                self.editing = Some(i);

                true
            },
            DocumentEditorMessage::EndEdit(block) => {
                let i = self.editing.take().unwrap();
                ctx.props().onblockedit.emit(DocumentBlockEdit(i, block));

                true
            },
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let len_blocks = ctx.props().document.blocks.len();

        if self.node_refs.len() != len_blocks {
            self.node_refs = vec![NodeRef::default(); len_blocks];

            true
        } else {
            true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let blocks = ctx.props().document.iter().enumerate().map(|(i, block)| {
            html! {
                <DocumentEditorBlock
                    block={ block.clone() }
                    oneditstart={ ctx.link().callback(move |()| DocumentEditorMessage::StartEdit(i)) }
                    oneditend={ ctx.link().callback(|block| DocumentEditorMessage::EndEdit(block)) }
                    insertbelow={ 
                        ctx.link().callback(
                            move |block| DocumentEditorMessage::Insert(DocumentBlockInsert(i + 1, block))
                        ) 
                    }
                    mergeabove={
                        if i > 0 {
                            ctx.link().callback(move |()| DocumentEditorMessage::Merge(DocumentBlockMerge(i - 1, i)))
                        } else {
                            Default::default()
                        }
                    }
                    edited={ self.editing == Some(i) }
                    ref={ self.node_refs[i].clone() }
                />
            }
        });

        let last_node = self.node_refs.last().unwrap().clone();

        let onclick = move |_| {
            let element = last_node.cast::<HtmlElement>().unwrap();
            //element.focus().unwrap();
        };

        html! {
            <div class="document-view">
                <div
                    class="document-content"
                    {onclick}
                >
                    { for blocks }
                </div>
            </div>
        }
    }
}

pub enum DocumentEditorBlockMessage {
    Focus(bool),
    Edit(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct DocumentEditorBlockProperties {
    #[prop_or_default]
    pub block: DocumentBlock,
    #[prop_or_default]
    pub oneditstart: Callback<()>,
    #[prop_or_default]
    pub oneditend: Callback<DocumentBlock>,
    #[prop_or_default]
    pub insertbelow: Callback<DocumentBlock>,
    #[prop_or_default]
    pub remove: Callback<()>,
    #[prop_or_default]
    pub mergeabove: Callback<()>,
    #[prop_or_default]
    pub edited: bool,
}

pub struct DocumentEditorBlock {
    node_ref: NodeRef,
}

impl Component for DocumentEditorBlock {
    type Message = DocumentEditorBlockMessage;
    type Properties = DocumentEditorBlockProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DocumentEditorBlockMessage::Focus(focus) => {
                let element = self.node_ref.cast::<HtmlElement>().unwrap();

                if focus && !ctx.props().edited {
                    ctx.props().oneditstart.emit(());
                } else if ctx.props().edited {
                    let block = DocumentBlock::new(element.inner_text());
                    ctx.props().oneditend.emit(block);
                }

                true
            }
            DocumentEditorBlockMessage::Edit(event) => {
                let mut caret = None;
                let selection = window().unwrap().get_selection().unwrap();
                if let Some(selection) = selection {
                    if let Ok(range) = selection.get_range_at(0) {
                        caret = Some(range.end_offset().unwrap());
                    }
                }

                match event.key().as_str() {
                    "Enter" if !event.shift_key() && caret.is_some() => {  
                        ctx.props().insertbelow.emit(DocumentBlock::default());
                    }
                    "Backspace" if caret == Some(0) => {
                        ctx.props().mergeabove.emit(());
                    }
                    _ => {}
                }

                false
            }
        }
    }    

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = if ctx.props().edited {
            classes!("document-block", "document-editor-block")
        } else {
            classes!("document-block", "document-view-block")
        };

        html! {
            <div
                {class}
                contenteditable="true"
                onfocus={ ctx.link().callback(|_| DocumentEditorBlockMessage::Focus(true)) }
                onfocusout={ ctx.link().callback(|_| DocumentEditorBlockMessage::Focus(false)) }
                onkeydown={ ctx.link().callback(DocumentEditorBlockMessage::Edit) }
                ref={ &self.node_ref }
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let element = self.node_ref.cast::<HtmlElement>().unwrap();
        if ctx.props().edited {
            element.set_inner_text(ctx.props().block.text());
            element.focus().unwrap();
        } else {  
            element.set_inner_html(&ctx.props().block.html());
        }
    }
}
