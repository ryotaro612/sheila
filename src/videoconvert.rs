// use gstreamer::prelude::*;
// use gstreamer::{ClockTime, Element, ElementFactory, MessageView, Pipeline, State};
// /// Tutorial: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/tree/main/tutorials/src/bin?ref_type=heads
// pub fn main() {
//     //tutorial_main();
//     gstreamer::init().unwrap();
//     let uri = "file:///home/ryotaro/Downloads/asuna.mp4";
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
//     let sink = gstreamer::ElementFactory::make("autovideosink")
//         .name("sink")
//         .build()
//         .expect("Could not create sink element.");

//     let pipeline = Pipeline::with_name("pipeline");
//     pipeline.add_many([&source, &convert, &sink]).unwrap();
//     Element::link_many([&convert, &sink]).unwrap();

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

//     pipeline
//         .set_state(State::Playing)
//         .expect("Unable to set the pipeline to the `Playing` state");

//     let bus = pipeline.bus().unwrap();

//     for msg in bus.iter_timed(ClockTime::NONE) {
//         match msg.view() {
//             MessageView::Eos(..) => {
//                 println!("received eos");
//                 // An EndOfStream event was sent to the pipeline, so exit
//                 break;
//             }
//             MessageView::Error(err) => {
//                 println!(
//                     "Error from {:?}: {} ({:?})",
//                     err.src().map(|s| s.path_string()),
//                     err.error(),
//                     err.debug()
//                 );
//                 break;
//             }
//             _ => (),
//         };
//     }
//     pipeline
//         .set_state(gstreamer::State::Null)
//         .expect("Unable to set the pipeline to the `Null` state");
// }
