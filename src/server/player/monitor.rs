///
use gdk4;
use gdk4::prelude::DisplayExt;
use gdk4::prelude::MonitorExt;
use gio::prelude::*;
use glib::Object;

pub(crate) fn detect_gdk_monitor(connector: &Option<String>) -> Result<gdk4::Monitor, String> {
    let monitors = detect_gdk_monitors()?;

    match connector {
        Some(c) => monitors
            .iter()
            .find(|m| m.connector().unwrap_or_default() == *c)
            .map(|m| m.clone())
            .ok_or(format!("monitor not found: {}", c)),
        None => monitors
            .get(0)
            .map(|m| m.clone())
            .ok_or("monitors were not found".to_string()),
    }
}

pub(crate) fn detect_connectors() -> Result<Vec<String>, String> {
    detect_gdk_monitors().map(|monitors| {
        monitors
            .iter()
            .filter_map(|m| m.connector())
            .map(|c| c.to_string())
            .collect()
    })
}

///
fn detect_gdk_monitors() -> Result<Vec<gdk4::Monitor>, String> {
    let display = detect_display()?;
    let monitors_list_model = display.monitors();
    let monitors: Vec<gdk4::Monitor> = monitors_list_model
        .iter()
        .map(|res| {
            let object: Object = res.map_err(|e| e.to_string())?;
            let monitor = object
                .downcast::<gdk4::Monitor>()
                .map_err(|_| "failed to downcast glib Object to Monitor".to_string())?;
            Ok(monitor)
        })
        .filter_map(|f: Result<gdk4::Monitor, String>| f.ok())
        .collect();
    Ok(monitors)
}

///
fn detect_display() -> Result<gdk4::Display, String> {
    //  gdk4::Display::open(None).unwrap(); or wayland-1 WAYLAND_DISPLAY env
    gdk4::Display::default().ok_or(String::from("failed to detect a display"))
}
