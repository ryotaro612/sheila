use crate::command::{self, Command};
mod monitor;
mod receiver;
pub(crate) mod state;
mod stream;
mod wallpaper;
mod window;
use crate::player::receiver as dr;
use gtk4::glib;
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
        let app = <gtk4::Application as wallpaper::Wallpaper>::new_application()?;

        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                let mut state = state::State::new();

                // let mut f = glib::clone!(
                //     #[weak]
                //     app,
                //     move |cmd: Command| {
                //         let mut is_shutdown_result = false;
                //         let result = state.execute(&app, &cmd).and_then(|value| {
                //             is_shutdown_result = value == shutdown_result();
                //             Ok(value)
                //         });
                //         let response = sender.send(result);
                //         if response.is_err() || is_shutdown_result {
                //             app.shutdown();
                //         }
                //     }
                // );
                loop {
                    match dr::ReceivedFuture::new(arc_receiver.clone()).await {
                        Ok(cmd) => {
                            let result = state.execute(&app, &cmd);
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
                "the wallpaper exits with unexpected status code: {}",
                code.value()
            )),
        }
    }
}
