use gdk4::prelude::MonitorExt;
use gtk4::{Application, Window};
use gtk4_layer_shell::LayerShell;

// Initializes a GTK window and attaches it to the specified monitor (connector).
// Returns an error if the monitor cannot be detected or the window cannot be created.
pub(crate) fn init_window(app: &Application, monitor: &gdk4::Monitor) -> Result<Window, String> {
    let window = Window::builder().application(app).build();
    window.init_layer_shell();
    window.set_layer(gtk4_layer_shell::Layer::Bottom);

    window.set_monitor(Some(monitor));
    window.set_anchor(gtk4_layer_shell::Edge::Left, true);
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);
    window.set_anchor(gtk4_layer_shell::Edge::Top, true);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    Ok(window)
}

// Returns the width and height of the monitor where the window is displayed.
// Returns an error if the monitor cannot be determined.
pub(crate) fn get_rectangle(window: &Window) -> Result<(i32, i32), String> {
    let rec = window.monitor()
        .map(|m| m.geometry())
        .ok_or_else(|| format!(
            "Unable to determine the monitor for the window: {:?}. The window may not be mapped to a monitor.",
            window
        ))?;

    Ok((rec.width(), rec.height()))
}
