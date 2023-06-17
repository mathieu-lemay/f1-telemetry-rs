use gtk::prelude::*;
use gtk::{InfoBar, Label, MessageType, ResponseType, Widget};

use f1_telemetry::packet::event::Event;

use crate::fmt;
use crate::models::GameState;

pub(super) struct EventsView {
    info_bar: InfoBar,
    message_label: Label,
}

impl EventsView {
    pub(super) fn new() -> Self {
        let info_bar = InfoBar::builder()
            .show_close_button(true)
            .no_show_all(true)
            .build();

        info_bar.connect_response(|bar, resp| {
            if resp == ResponseType::Close {
                bar.hide();
            }
        });

        let message_label = Label::builder().name("events").build();

        info_bar.content_area().add(&message_label);

        Self {
            info_bar,
            message_label,
        }
    }

    pub(super) fn update(&self, game_state: &GameState) {
        let event = &game_state.event_info.event;

        if let Event::Buttons(_) = event {
            return;
        }

        self.message_label.hide();
        self.info_bar.hide();

        self.message_label
            .set_label(&fmt::format_event_info(&game_state.event_info));
        self.info_bar
            .set_message_type(get_message_type(&game_state.event_info.event));

        self.message_label.show();
        self.info_bar.show();
    }

    pub(super) fn widget(&self) -> &impl IsA<Widget> {
        &self.info_bar
    }
}

fn get_message_type(event: &Event) -> MessageType {
    match event {
        Event::Penalty(_) => MessageType::Error,
        Event::DRSDisabled | Event::Retirement(_) => MessageType::Warning,
        _ => MessageType::Info,
    }
}
