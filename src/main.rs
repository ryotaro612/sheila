use gstreamer::prelude::*;
use gstreamer::{ClockTime, Element, ElementFactory, MessageView, Pipeline, State};
use gstreamer_video;
use gtk4::gdk::Paintable;
use gtk4::prelude::*;
use gtk4::{gdk, Picture};
use std::cell::RefCell;
fn create_ui(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(app);
    window.set_default_size(640, 480);

    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    let picture = gtk4::Picture::new();

    let pipeline = gstreamer::Pipeline::new();

    let gtksink = gstreamer::ElementFactory::make("gtk4paintablesink")
        .build()
        .unwrap();

    let paintable = gtksink.property::<gdk::Paintable>("paintable");

    let (src, sink) = {
        println!("dogggg");
        let src = gstreamer::ElementFactory::make("videotestsrc")
            .build()
            .unwrap();

        let sink = gstreamer::Bin::default();
        let convert = gstreamer::ElementFactory::make("videoconvert")
            .build()
            .unwrap();

        sink.add(&convert).unwrap();
        sink.add(&gtksink).unwrap();
        convert.link(&gtksink).unwrap();

        sink.add_pad(
            &gstreamer::GhostPad::with_target(&convert.static_pad("sink").unwrap()).unwrap(),
        )
        .unwrap();

        (src, sink.upcast())
    };

    pipeline.add_many([&src, &sink]).unwrap();
    Element::link_many([&src, &sink]).unwrap();

    picture.set_paintable(Some(&paintable));
    vbox.append(&picture);

    window.set_child(Some(&vbox));
    window.present();

    app.add_window(&window);

    let pipeline_weak = pipeline.downgrade();
    let timeout_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let Some(pipeline) = pipeline_weak.upgrade() else {
            return glib::ControlFlow::Break;
        };

        let position = pipeline.query_position::<gstreamer::ClockTime>();

        glib::ControlFlow::Continue
    });

    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let app_weak = app.downgrade();
    let bus_watch = bus
        .add_watch_local(move |_, msg| {
            use gstreamer::MessageView;

            let Some(app) = app_weak.upgrade() else {
                return glib::ControlFlow::Break;
            };

            match msg.view() {
                MessageView::Eos(..) => app.quit(),
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    app.quit();
                }
                _ => (),
            };

            glib::ControlFlow::Continue
        })
        .expect("Failed to add bus watch");

    let timeout_id = RefCell::new(Some(timeout_id));
    let pipeline = RefCell::new(Some(pipeline));
    let bus_watch = RefCell::new(Some(bus_watch));
    app.connect_shutdown(move |_| {
        window.close();

        drop(bus_watch.borrow_mut().take());
        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline
                .set_state(gstreamer::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
        }

        if let Some(timeout_id) = timeout_id.borrow_mut().take() {
            timeout_id.remove();
        }
    });
}

fn main() -> glib::ExitCode {
    gstreamer::init().unwrap();
    gtk4::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    let app = gtk4::Application::new(None::<&str>, gio::ApplicationFlags::FLAGS_NONE);

    app.connect_activate(create_ui);
    let res = app.run();

    unsafe {
        gstreamer::deinit();
    }

    res
}

pub fn main2() {
    gstreamer::init().unwrap();
    gtk4::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("dev.nryotaro.sheila")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}

// https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.py
fn init_gstreamer() -> (Paintable, Pipeline) {
    let uri = "file:///home/ryotaro/Downloads/asuna.mp4";
    let source = gstreamer::ElementFactory::make("uridecodebin")
        .name("source")
        // Set the URI to play
        .property("uri", uri)
        .build()
        .expect("Could not create uridecodebin element.");
    let convert = gstreamer::ElementFactory::make("videoconvert")
        .name("convert")
        .build()
        .expect("Could not create convert element.");
    let gtksink = ElementFactory::make("gtk4paintablesink")
        .name("sink")
        .build()
        .expect("Could not create sink element");
    //	ElementFactory::make("gtk4paintablesink", "sink");

    let pipeline = Pipeline::with_name("pipeline");
    pipeline.add_many([&source, &convert, &gtksink]).unwrap();
    Element::link_many([&convert, &gtksink]).unwrap();
    println!("mid init");
    source.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        src.downcast_ref::<gstreamer::Bin>()
            .unwrap()
            .debug_to_dot_file_with_ts(gstreamer::DebugGraphDetails::all(), "pad-added");

        let sink_pad = convert
            .static_pad("sink")
            .expect("Failed to get static sink pad from convert");
        if sink_pad.is_linked() {
            println!("We are already linked. Ignoring.");
            return;
        }

        let new_pad_caps = src_pad
            .current_caps()
            .expect("Failed to get caps of new pad.");
        let size = new_pad_caps.size();
        println!("{}", size);
        let new_pad_struct = new_pad_caps
            .structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.name();
        println!("{}", new_pad_type);
        let is_video = new_pad_type.starts_with("video/x-raw");
        if !is_video {
            println!("It has type {new_pad_type} which is not raw video. Ignoring.");
            return;
        }

        let res = src_pad.link(&sink_pad);
        if res.is_err() {
            println!("Type is {new_pad_type} but link failed.");
        } else {
            println!("Link succeeded (type {new_pad_type}).");
        }
    });
    println!("before playing");
    // pipeline
    //     .set_state(State::Playing)
    //     .expect("Unable to set the pipeline to the `Playing` state");

    let paintable = gtksink.property::<gdk::Paintable>("paintable");
    (paintable, pipeline)
}

// When the application is launched…
fn on_activate(application: &gtk4::Application) {
    // … create a new window …
    let window = gtk4::ApplicationWindow::new(application);
    // … with a button in it …
    let (paintable, pipeline) = init_gstreamer();
    println!("doge");
    let picture = gtk4::Picture::for_paintable(&paintable);
    // picture.set_halign(gtk4::Align::Center);
    // picture.set_size_request(200, 200);
    //picture.set_paintable(Some(&paintable));

    window.set_child(Some(&picture));
    window.present();
    println!("after present");

    pipeline
        .set_state(State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                // An EndOfStream event was sent to the pipeline, so exit
                break;
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        };
    }
    println!("after playing");
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
