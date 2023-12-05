use gst::prelude::*;
use gstreamer as gst;
use gstreamer::{Bin, ElementFactory, Pipeline, State};
use gstreamer_video as gst_video;
use gtk4;
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, Orientation, Picture};
use gtk4::{Video, Window};
use std::cell::RefCell;
mod video_player_window;
use video_player_window::VideoPlayerWindow;

// Reference
// https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/blob/main/video/gtk4/examples/gtksink.rs?ref_type=heads
fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_default_size(640, 480);

    let vbox = gtk4::Box::new(Orientation::Vertical, 0);
    let picture = Picture::new();

    let pipeline = gst::Pipeline::new();

    let overlay = gst::ElementFactory::make("clockoverlay")
        .property("font-desc", "Monospace 42")
        .build()
        .unwrap();

    let gtksink = gst::ElementFactory::make("gtk4paintablesink")
        .build()
        .unwrap();

    let paintable = gtksink.property::<gdk::Paintable>("paintable");

    // TODO: future plans to provide a bin-like element that works with less setup
    let (src, sink) = {
        let src = gst::ElementFactory::make("videotestsrc").build().unwrap();
        // let src = ElementFactory::make("uridecodebin")
        //     .property("uri", "file:///home/ryotaro/Downloads/asuna.mp4")
        //     .build()
        //     .unwrap();
        let sink = gst::Bin::default();
        let convert = gst::ElementFactory::make("videoconvert").build().unwrap();

        sink.add(&convert).unwrap();
        sink.add(&gtksink).unwrap();
        convert.link(&gtksink).unwrap();

        sink.add_pad(&gst::GhostPad::with_target(&convert.static_pad("sink").unwrap()).unwrap())
            .unwrap();

        (src, sink.upcast())
    };

    pipeline.add_many([&src, &overlay, &sink]).unwrap();
    let caps = gst_video::VideoCapsBuilder::new()
        .width(640)
        .height(480)
        .any_features()
        .build();

    src.link_filtered(&overlay, &caps).unwrap();
    overlay.link(&sink).unwrap();

    picture.set_paintable(Some(&paintable));
    vbox.append(&picture);

    window.set_child(Some(&vbox));
    window.show();

    app.add_window(&window);

    let pipeline_weak = pipeline.downgrade();

    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let app_weak = app.downgrade();
    let bus_watch = bus
        .add_watch_local(move |_, msg| {
            use gst::MessageView;

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

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("Sheila")
    //     .build();
    // // let src = ElementFactory::make("uridecodebin")
    // //     .property("uri", "file:///home/ryotaro/Downloads/asuna.mp4")
    // //     .build()
    // //     .unwrap();
    // let src = gst::ElementFactory::make("videotestsrc").build().unwrap();
    // let picture = Picture::new();

    // let gtksink = ElementFactory::make("gtk4paintablesink").build().unwrap();
    // let paintable = gtksink.property::<gdk::Paintable>("paintable");
    // let sink = Bin::default();
    // let convert = ElementFactory::make("videoconvert").build().unwrap();
    // sink.add(&convert).unwrap();
    // sink.add(&gtksink).unwrap();
    // convert.link(&gtksink).unwrap();
    // sink.add_pad(&gst::GhostPad::with_target(&convert.static_pad("sink").unwrap()).unwrap())
    //     .unwrap();

    // picture.set_paintable(Some(&paintable));
    // let vbox = gtk4::Box::new(Orientation::Vertical, 0);
    // vbox.append(&picture);

    // window.set_child(Some(&vbox));

    // let pipeline = Pipeline::new();
    // pipeline.add_many([&src, &sink.upcast()]).unwrap();
    // pipeline
    //     .set_state(State::Playing)
    //     .expect("Unable to set the pipeline to the `Playing` state");

    // let bus = pipeline.bus().unwrap();
    // let app_weak = app.downgrade();
    // let bus_watch = bus
    //     .add_watch_local(move |_, msg| {
    //         use gst::MessageView;

    //         let Some(app) = app_weak.upgrade() else {
    //             return glib::ControlFlow::Break;
    //         };

    //         match msg.view() {
    //             MessageView::Eos(..) => app.quit(),
    //             MessageView::Error(err) => {
    //                 println!(
    //                     "Error from {:?}: {} ({:?})",
    //                     err.src().map(|s| s.path_string()),
    //                     err.error(),
    //                     err.debug()
    //                 );
    //                 app.quit();
    //             }
    //             _ => (),
    //         };

    //         glib::ControlFlow::Continue
    //     })
    //     .expect("Failed to add bus watch");
    // window.present();
}

fn main() -> glib::ExitCode {
    gst::init().unwrap();
    gtk4::init().unwrap();
    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    let app = gtk4::Application::builder()
        .application_id("dev.nryotaro.sheila")
        .build();

    app.connect_activate(build_ui);
    let res = app.run();

    unsafe {
        gst::deinit();
    }

    res
}
