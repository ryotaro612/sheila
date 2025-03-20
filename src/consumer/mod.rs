use crate::command;
use std::result;
use std::sync::mpsc;

pub(crate) struct Consumer<'a> {
    command_receiver: &'a mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<result::Result<(), String>>,
}

impl<'a> Consumer<'a> {
    pub(crate) fn new(
        command_receiver: &'a mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<result::Result<(), String>>,
    ) -> Self {
        Consumer {
            command_receiver,
            result_sender,
        }
    }

    pub(crate) fn run(&self) -> result::Result<(), String> {
        Ok(())
    }
}
