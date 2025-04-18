/**
 *
 */
use gdk4;
use gdk4::prelude::DisplayExt;
use gdk4::prelude::MonitorExt;
use gio::prelude::*;
use glib::Object;

fn detect_gdk_monitors() -> Result<Vec<gdk4::Monitor>, String> {
    let display = detect_display()?;
    let monitors_list_model = display.monitors();
    let monitors: Vec<gdk4::Monitor> = monitors_list_model
        .iter()
        .map(|res| {
            let object: Object = res.map_err(|e| e.to_string())?;
            let monitor = object
                .downcast::<gdk4::Monitor>()
                .map_err(|_| "failed to downcast a glib object to a monitor".to_string())?;
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
 * Detects monitors and returns a list of them.
 */
pub(crate) fn detect_monitors() -> Result<Monitors, String> {
    let display = detect_display()?;
    let monitors_list_model = display.monitors();
    let monitors: Vec<Monitor> = monitors_list_model
        .iter()
        .map(|res| {
            let object: Object = res.map_err(|e| e.to_string())?;

            let monitor = object
                .downcast::<gdk4::Monitor>()
                .map_err(|_| String::from("failed to downcast a glib object to a monitor"))?;

            let connector: String = monitor
                .connector()
                .ok_or(String::from("failed to get a connector"))?
                .into();
            let geometry = monitor.geometry();

            let res: Result<Monitor, String> = Ok(Monitor {
                connector,
                x: geometry.x(),
                y: geometry.y(),
                width: geometry.width(),
                height: geometry.height(),
            });
            res
        })
        .filter_map(|f| f.ok())
        .collect();

    match monitors.len() {
        0 => {
            return Err(String::from("no monitors detected"));
        }
        _ => {
            return Ok(Monitors { monitors });
        }
    }
}

#[derive(Debug)]
pub(crate) struct Monitor {
    connector: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug)]
pub(crate) struct Monitors {
    monitors: Vec<Monitor>,
}

/**
 *
 */
impl Monitors {
    /**
     * Returns the primary monitor.
     */
    pub(crate) fn first(&self) -> Option<String> {
        if 0 < self.monitors.len() {
            Some(self.monitors[0].connector.clone())
        } else {
            None
        }
    }
}

/**
 *
 */
fn detect_display() -> Result<gdk4::Display, String> {
    //  gdk4::Display::open(None).unwrap(); or wayland-1 WAYLAND_DISPLAY env
    gdk4::Display::default().ok_or(String::from("failed to detect a display"))
}
// fn temp() {
//     //let a = gdk4::Display::default();
//     //let display = gdk4_sys::gdk_display_get_default();
//     // gdk_monitor_get_geometry();
//     //gdk4_sys::
//     let c = gdk4::Display::open(None).unwrap();
//     println!("{:?}", c);
//     let display = gdk4::Display::default().unwrap();
//     let monitors = display.monitors();
//     let n = monitors.n_items();
//     println!("num: {n}");
//     let monitor = monitors
//         .item(1)
//         .unwrap()
//         .downcast::<gdk4::Monitor>()
//         .unwrap();

//     println!(
//         "monitor-size: {:?} {:?}",
//         monitor.connector(),
//         monitor.geometry()
//     );
// }
