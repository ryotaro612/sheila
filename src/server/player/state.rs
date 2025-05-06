use super::{playlist::Playlist, stream};
use gtk4::Picture;
use serde;
use std::collections::HashMap;
///
///
pub(crate) struct State {
    app_running: bool,
    playing: HashMap<String, stream::Stream>,
}

impl State {
    ///
    pub(crate) fn new() -> Self {
        State {
            app_running: false,
            playing: HashMap::new(),
        }
    }

    pub(crate) fn set_app_running(&mut self, running: bool) {
        self.app_running = running;
    }
    pub(crate) fn is_app_running(&self) -> bool {
        self.app_running
    }

    pub(crate) fn play_stream(&mut self, connector: &str) -> Result<(), String> {
        let stream = self
            .playing
            .get(connector)
            .ok_or(format!("Stream was not found at {}", connector))?;

        stream.play().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Returns Err if another movie is playing at connector.
    pub(crate) fn add_stream(
        &mut self,
        connector: &str,
        playlist: &Playlist,
        width: u32,
        height: u32,
        on_error: impl Fn() -> () + 'static,
    ) -> Result<gtk4::Picture, String> {
        if self.playing.get(connector).is_some() {
            return Err(format!("Another playlist is on {}", connector));
        }
        let stream = stream::Stream::new(playlist.clone(), width, height, on_error)
            .map_err(|e| e.to_string())?;

        let picture = Picture::for_paintable(&stream.paintable());
        self.playing.insert(connector.to_string(), stream);
        Ok(picture)
    }

    pub(crate) fn stop_stream(&mut self, connector: &str) {
        self.playing.get(connector).map(|s| {
            if let Err(e) = s.stop() {
                log::error!("Error stopping stream: {}", e);
            }
        });

        self.playing.remove(connector);
    }

    pub(crate) fn lookup_plying_movies(&self) -> Vec<PlayingMovie> {
        self.playing
            .iter()
            .map(|(connector, stream)| PlayingMovie {
                connector: connector.clone(),
                file: stream.get_playing_file(),
            })
            .collect()
    }
}

#[derive(serde::Serialize, Debug)]
pub(crate) struct PlayingMovie {
    connector: String,
    file: String,
}
