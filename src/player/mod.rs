use crate::command::{self, Command};
mod monitor;
mod receiver;
mod state;
mod stream;
mod wallpaper;
mod window;
use crate::player::receiver as dr;

use gtk4::glib;
use std::result;
use std::sync::{self, mpsc};
use wallpaper::Wallpaper;
pub(crate) struct Drawer<'a> {
    command_receiver: mpsc::Receiver<command::Command>,
    result_sender: &'a mpsc::Sender<Result<serde_json::Value, command::ErrorReason>>,
}

impl<'a> Drawer<'a> {
    pub(crate) fn new(
        command_receiver: mpsc::Receiver<command::Command>,
        result_sender: &'a mpsc::Sender<Result<serde_json::Value, command::ErrorReason>>,
    ) -> Self {
        Drawer {
            command_receiver,
            result_sender,
        }
    }

    /**
     *
     */
    pub(crate) fn run(self) -> result::Result<(), String> {
        let sender = self.result_sender.clone();
        let arc_receiver = sync::Arc::new(sync::Mutex::new(self.command_receiver));
        let app = <gtk4::Application as wallpaper::Wallpaper>::new_application()?;

        glib::spawn_future_local(glib::clone!(
            #[weak]
            app,
            async move {
                let mut state = state::State::new();

                let mut f = glib::clone!(
                    #[weak]
                    app,
                    move |cmd: Command| {
                        let res = state.execute(&app, &cmd);
                        if let Err(e) = sender.send(res) {
                            log::error!("disconnected: {e}");
                            state
                                .execute(&app, &Command::Stop { monitor: None })
                                .unwrap();
                        }
                    }
                );
                loop {
                    let res = dr::ReceivedFuture::new(arc_receiver.clone()).await;
                    match res {
                        Ok(cmd) => f(cmd),
                        Err(r_err) => {
                            // https://doc.rust-lang.org/std/sync/mpsc/struct.RecvError.html
                            log::debug!("disconnected: {r_err}");
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
