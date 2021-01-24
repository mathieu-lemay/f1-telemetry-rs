use gtk::prelude::*;
use gtk::{InfoBarBuilder, LabelBuilder, MessageType, ResponseType};

use f1_telemetry::packet::event::Event;

use crate::fmt;
use crate::models::GameState;

pub(super) struct EventsView {
    pub(super) info_bar: gtk::InfoBar,
    message_label: gtk::Label,
}

impl EventsView {
    pub(super) fn new() -> Self {
        let info_bar = InfoBarBuilder::new()
            .show_close_button(true)
            .no_show_all(true)
            .build();

        info_bar.connect_response(|bar, resp| {
            if resp == ResponseType::Close {
                bar.hide();
            }
        });

        let message_label = LabelBuilder::new().name("events").build();

        info_bar.get_content_area().add(&message_label);

        Self {
            info_bar,
            message_label,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        self.message_label.hide();
        self.info_bar.hide();

        self.message_label
            .set_label(&fmt::format_event_info(&game_state.event_info));
        self.info_bar
            .set_message_type(get_message_type(&game_state.event_info.event));

        self.message_label.show();
        self.info_bar.show();
    }
}

fn get_message_type(event: &Event) -> MessageType {
    match event {
        Event::Penalty(_) => MessageType::Error,
        Event::DRSDisabled | Event::Retirement(_) => MessageType::Warning,
        _ => MessageType::Info,
    }
}
