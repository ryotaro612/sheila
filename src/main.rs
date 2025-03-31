use log;
use std::process::{self, exit};

use gdk4::prelude::{DisplayExt, MonitorExt};
use gdk4_sys;
use gio::prelude::*;
mod client;
mod command;
mod draw;
mod logger;
mod parser;
mod server;

fn main() {
    temp();
    let args: Vec<String> = std::env::args().collect();
    // https://docs.rs/clap/latest/clap/type.Error.html
    let cli = parser::parse(args).map_err(|err| err.exit()).unwrap();

    logger::init_log(cli.verbose);

    match cli.command {
        parser::Commands::Server => server::run(cli.socket),
        parser::Commands::Client(client_args) => client::run(cli.socket, client_args.command),
    }
    .unwrap_or_else(|e| {
        log::error!("error: {e}");
        process::exit(1);
    });
}

fn temp() {
    //let a = gdk4::Display::default();
    //let display = gdk4_sys::gdk_display_get_default();
    // gdk_monitor_get_geometry();
    //gdk4_sys::
    let c = gdk4::Display::open(None).unwrap();
    println!("{:?}", c);
    let display = gdk4::Display::default().unwrap();
    let monitors = display.monitors();
    let n = monitors.n_items();
    println!("num: {n}");
    let monitor = monitors
        .item(1)
        .unwrap()
        .downcast::<gdk4::Monitor>()
        .unwrap();

    println!(
        "monitor-size: {:?} {:?}",
        monitor.connector(),
        monitor.geometry()
    );
}

// use gstreamer::prelude::*;
// use gstreamer::{ClockTime, Element, ElementFactory, MessageView, Pipeline, State};
// use gstreamer_video;
// use gtk4::gdk::Paintable;
// use gtk4::prelude::*;
// use gtk4::{gdk, Picture};
// use std::cell::RefCell;
// fn create_ui(app: &gtk4::Application) {
//     let window = gtk4::ApplicationWindow::new(app);
//     window.set_default_size(640, 480);

//     let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
//     let picture = gtk4::Picture::new();

//     let pipeline = gstreamer::Pipeline::new();

//     let gtksink = gstreamer::ElementFactory::make("gtk4paintablesink")
//         .name("sink")
//         .build()
//         .unwrap();

//     let paintable = gtksink.property::<gdk::Paintable>("paintable");

//     let uri = "file:///home/ryotaro/Downloads/fubuki.mp4";
//     //let uri = "file:///home/ryotaro/Downloads/asuna.mp4";
//     let source = gstreamer::ElementFactory::make("uridecodebin")
//         .name("source")
//         // Set the URI to play
//         .property("uri", uri)
//         .build()
//         .expect("Could not create uridecodebin element.");

//     let convert = gstreamer::ElementFactory::make("videoconvert")
//         .name("convert")
//         .build()
//         .expect("Could not create convert element.");

//     // pipeline.add_many([&src, &sink]).unwrap();
//     // Element::link_many([&src, &sink]).unwrap();

//     pipeline.add_many([&source, &convert, &gtksink]).unwrap();
//     Element::link_many([&convert, &gtksink]).unwrap();
//     source.connect_pad_added(move |src, src_pad| {
//         println!("Received new pad {} from {}", src_pad.name(), src.name());

//         src.downcast_ref::<gstreamer::Bin>()
//             .unwrap()
//             .debug_to_dot_file_with_ts(gstreamer::DebugGraphDetails::all(), "pad-added");

//         let sink_pad = convert
//             .static_pad("sink")
//             .expect("Failed to get static sink pad from convert");
//         if sink_pad.is_linked() {
//             println!("We are already linked. Ignoring.");
//             return;
//         }

//         let new_pad_caps = src_pad
//             .current_caps()
//             .expect("Failed to get caps of new pad.");
//         let size = new_pad_caps.size();
//         println!("{}", size);
//         let new_pad_struct = new_pad_caps
//             .structure(0)
//             .expect("Failed to get first structure of caps.");
//         let new_pad_type = new_pad_struct.name();
//         println!("{}", new_pad_type);
//         let is_video = new_pad_type.starts_with("video/x-raw");
//         if !is_video {
//             println!("It has type {new_pad_type} which is not raw video. Ignoring.");
//             return;
//         }

//         let res = src_pad.link(&sink_pad);
//         if res.is_err() {
//             println!("Type is {new_pad_type} but link failed.");
//         } else {
//             println!("Link succeeded (type {new_pad_type}).");
//         }
//     });

//     picture.set_paintable(Some(&paintable));
//     vbox.append(&picture);

//     window.set_child(Some(&vbox));
//     window.present();

//     app.add_window(&window);

//     let pipeline_weak = pipeline.downgrade();
//     let timeout_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
//         let Some(pipeline) = pipeline_weak.upgrade() else {
//             return glib::ControlFlow::Break;
//         };

//         let position = pipeline.query_position::<gstreamer::ClockTime>();

//         glib::ControlFlow::Continue
//     });

//     let bus = pipeline.bus().unwrap();

//     pipeline
//         .set_state(gstreamer::State::Playing)
//         .expect("Unable to set the pipeline to the `Playing` state");

//     let app_weak = app.downgrade();
//     let bus_watch = bus
//         .add_watch_local(move |_, msg| {
//             use gstreamer::MessageView;

//             let Some(app) = app_weak.upgrade() else {
//                 return glib::ControlFlow::Break;
//             };

//             match msg.view() {
//                 MessageView::Eos(..) => app.quit(),
//                 MessageView::Error(err) => {
//                     println!(
//                         "Error from {:?}: {} ({:?})",
//                         err.src().map(|s| s.path_string()),
//                         err.error(),
//                         err.debug()
//                     );
//                     app.quit();
//                 }
//                 _ => (),
//             };

//             glib::ControlFlow::Continue
//         })
//         .expect("Failed to add bus watch");

//     let timeout_id = RefCell::new(Some(timeout_id));
//     let pipeline = RefCell::new(Some(pipeline));
//     let bus_watch = RefCell::new(Some(bus_watch));
//     app.connect_shutdown(move |_| {
//         window.close();

//         drop(bus_watch.borrow_mut().take());
//         if let Some(pipeline) = pipeline.borrow_mut().take() {
//             pipeline
//                 .set_state(gstreamer::State::Null)
//                 .expect("Unable to set the pipeline to the `Null` state");
//         }

//         if let Some(timeout_id) = timeout_id.borrow_mut().take() {
//             timeout_id.remove();
//         }
//     });
// }

// fn main() -> glib::ExitCode {
//     gstreamer::init().unwrap();
//     gtk4::init().unwrap();

//     gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

//     let app = gtk4::Application::new(None::<&str>, gio::ApplicationFlags::FLAGS_NONE);

//     app.connect_activate(create_ui);
//     let res = app.run();

//     unsafe {
//         gstreamer::deinit();
//     }

//     res
// }
