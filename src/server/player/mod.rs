use crate::command::{self, Command};
use std::sync::{Arc, Mutex};
mod monitor;
mod state;
mod status;
mod stream;
mod wallpaper;
mod window;
use glib::clone::Downgrade;
use gtk4::glib;
pub(crate) mod operation;
mod playlist;
mod receiver;
use std::result;
use std::sync::{self, mpsc};
use wallpaper::Wallpaper;
pub(crate) struct Player<'a> {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<Result<serde_json::Value, command::ErrorReason>>,
}

impl<'a> Player<'a> {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<Result<serde_json::Value, command::ErrorReason>>,
    ) -> Self {
        Player {
            command_receiver,
            result_sender,
        }
    }

    ///
    pub(crate) fn run(self) -> result::Result<(), String> {
        let sender = self.result_sender.clone();
        let arc_receiver = sync::Arc::new(sync::Mutex::new(self.command_receiver));

        let state = Arc::new(Mutex::new(state::State::new()));
        let app = wallpaper::new_application(&state)?;

        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                loop {
                    match receiver::ReceivedFuture::new(arc_receiver.clone()).await {
                        Ok(cmd) => {
                            let result = operation::operate(&state.downgrade(), &app, &cmd);
                            let response = sender.send(result);
                            if response.is_err() {
                                app.shutdown();
                            }
                            if response.is_err() || cmd == Command::Shutdown {
                                break;
                            }
                        }
                        Err(_) => {
                            app.shutdown();
                            break;
                        }
                    }
                }
            }
        ));
        match app.start() {
            glib::ExitCode::SUCCESS => Ok(()),
            code => Err(format!(
                "Player exits with unexpected status code: {}",
                code.value()
            )),
        }
    }
}
