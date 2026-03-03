use crate::ui::{MessageType, UI};
use gtk::prelude::*;

pub struct GUI {
    window: gtk::Window,
}

impl GUI {
    pub fn new(window: gtk::Window) -> Self {
        GUI { window }
    }
}

impl UI for GUI {
    fn show_message(&self, message_type: MessageType, message: &str, title: String) {
        let dialog_msg_type = match message_type {
            MessageType::Info => gtk::MessageType::Info,
            MessageType::Warning => gtk::MessageType::Warning,
            MessageType::Error => gtk::MessageType::Error,
        };

        let dialog = gtk::MessageDialog::builder()
            .transient_for(&self.window)
            .message_type(dialog_msg_type)
            .text(message)
            .title(title)
            .modal(true)
            .buttons(gtk::ButtonsType::Ok)
            .build();
        dialog.connect_response(|dialog, _| dialog.close());

        dialog.show();
        // block until user responds
        dialog.run();
        // we are required to close/hide manually according to the docs
        dialog.close();
    }
}

pub fn run_command(command: &str, escalate: bool) -> bool {
    termin::run_command(command, escalate)
}
