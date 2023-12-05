use gstreamer::prelude::*;
use gstreamer::{Bin, ClockTime, ElementFactory, Pipeline, State};
use gstreamer_video as gst_video;
use gtk4;
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib, Application, ApplicationWindow, Orientation, Picture};
use gtk4::{Video, Window};
use std::cell::RefCell;
mod video_player_window;
use gstreamer::MessageView;
use video_player_window::VideoPlayerWindow;

mod tutorial1;
fn tutorial1() {
    gstreamer::init().unwrap();

    let pipeline: Pipeline =
        gstreamer::parse_launch("playbin uri=file:///home/ryotaro/Downloads/asuna.mp4")
            .unwrap()
            .dynamic_cast()
            .unwrap();

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
}

fn main() {}
