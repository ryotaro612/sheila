use super::stream;
use std::collections::HashMap;
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

    pub(crate) fn add_stream(&mut self, connector: &str, stream: &stream::Stream) {
        self.playing.insert(connector.to_string(), stream.clone());
    }

    pub(crate) fn stop_stream(&mut self, connector: &str) {
        self.playing.get(connector).map(|s| {
            s.stop().unwrap();
        });

        self.playing.remove(connector);
    }
}
