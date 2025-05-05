use gdk4::prelude::MonitorExt;
use gtk4::{Application, Window};
use gtk4_layer_shell::{Edge, LayerShell};

/// Initializes a GTK window and attaches it to the specified monitor (connector).
/// Returns an error if the monitor cannot be detected or the window cannot be created.
pub(crate) fn init_window(app: &Application, monitor: &gdk4::Monitor) -> Result<Window, String> {
    let window = Window::builder().application(app).build();
    // GTK4 warns: "Make sure you called gtk_layer_init_for_window()" before the method invocation.
    window.init_layer_shell();
    window.set_layer(gtk4_layer_shell::Layer::Bottom);
    window.set_monitor(Some(monitor));
    for edge in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        window.set_anchor(edge, true);
    }
    Ok(window)
}

/// Returns the width and height of the monitor where the window is displayed.
/// Returns an error if the monitor cannot be determined.
pub(crate) fn get_rectangle(window: &Window) -> Result<(u32, u32), String> {
    let rec = window.monitor()
        .map(|m| m.geometry())
        .ok_or_else(|| format!(
            "Unable to determine the monitor for the window: {:?}. The window may not be mapped to a monitor.",
            window
        ))?;

    Ok((rec.width() as u32, rec.height() as u32))
}
