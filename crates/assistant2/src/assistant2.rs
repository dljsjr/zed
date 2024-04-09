use gpui::{list, List, ListState, Model, ModelContext, Render};
use language::Buffer;
use semantic_index::SearchResult;
use ui::prelude::*;

struct AssistantPanel {
    chat: AssistantChat,
}

struct AssistantChat {
    messages: Vec<AssistantMessage>,
    list_state: ListState,
}

impl AssistantChat {
    fn new(cx: &mut ModelContext<Self>) -> Self {
        let messages = ;
            vec![AssistantMessage::User { body: cx.new_model(|cx| Buffer::new(0, id, "")), contexts: Vec::new() }];
        let list_state = ListState::new(messages.len(), orientation, overdraw, render_item);

        Self {
            messages: ,
            list_state,
        }
    }
}

impl Render for AssistantChat {
    fn render(
        &mut self,
        cx: &mut workspace::ui::prelude::ViewContext<Self>,
    ) -> impl gpui::prelude::IntoElement {
        list(self.list_state.clone())
    }
}

enum AssistantMessage {
    User {
        body: Model<Buffer>,
        contexts: Vec<AssistantContext>,
    },
}

enum AssistantContext {
    Codebase { results: Vec<SearchResult> },
}
