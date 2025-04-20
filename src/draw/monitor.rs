/**
 *
 */
use gdk4;
use gdk4::prelude::DisplayExt;
use gdk4::prelude::MonitorExt;
use gio::prelude::*;
use glib::Object;

/**
 *
 */
fn detect_gdk_monitors() -> Result<Vec<gdk4::Monitor>, String> {
    let display = detect_display()?;
    let monitors_list_model = display.monitors();
    let monitors: Vec<gdk4::Monitor> = monitors_list_model
        .iter()
        .map(|res| {
            let object: Object = res.map_err(|e| e.to_string())?;
            let monitor = object
                .downcast::<gdk4::Monitor>()
                .map_err(|_| "failed to downcast n glib Object to an gdk4::Monitor".to_string())?;
            Ok(monitor)
        })
        .filter_map(|f: Result<gdk4::Monitor, String>| f.ok())
        .collect();
    Ok(monitors)
}
pub(crate) fn detect_gdk_monitor(connector: &str) -> Result<gdk4::Monitor, String> {
    let monitors = detect_gdk_monitors()?;
    let monitor = monitors
        .iter()
        .find(|m| m.connector().unwrap_or_default() == connector)
        .map(|m| m)
        .ok_or(format!("monitor not found: {}", connector))?;

    Ok(monitor.clone())
}

pub(crate) fn detect_primary_monitor() -> Result<String, String> {
    let monitors = detect_gdk_monitors()?;
    let c: Vec<String> = monitors
        .iter()
        .filter_map(|m| m.connector())
        .map(|e| e.to_string())
        .collect();
    c.get(0)
        .map(|x| x.to_string())
        .ok_or("there aren't any monitors".to_string())
}

/**
 *
 */
fn detect_display() -> Result<gdk4::Display, String> {
    //  gdk4::Display::open(None).unwrap(); or wayland-1 WAYLAND_DISPLAY env
    gdk4::Display::default().ok_or(String::from("failed to detect a display"))
}
