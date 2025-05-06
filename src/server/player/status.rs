use super::monitor::detect_connectors;
use super::state;
use super::wallpaper;
use serde;
use std::sync::{Mutex, Weak};

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Status {
    app_running: bool,
    connectors: Vec<String>,
    n_windows: usize,
    movies: Vec<state::PlayingMovie>,
}

pub(crate) fn lookup_status(
    state_weak: &Weak<Mutex<state::State>>,
    wallpaper: &impl wallpaper::Wallpaper,
) -> Result<Status, String> {
    let status_arc = state_weak.upgrade().ok_or("failed to upgrade state")?;
    let state = status_arc.lock().map_err(|e| e.to_string())?;
    if !state.is_app_running() {
        return Ok(Status {
            app_running: false,
            connectors: vec![],
            n_windows: 0,
            movies: vec![],
        });
    }
    let connectors = detect_connectors()?;
    let n_windows = wallpaper.count_windows();
    let movies = state.lookup_plying_movies();

    Ok(Status {
        app_running: true,
        connectors,
        n_windows,
        movies,
    })
}
