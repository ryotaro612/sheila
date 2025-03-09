// mod imp;

// use gtk4::{gio, glib, prelude::*, subclass::prelude::*};

// gtk4::glib::wrapper! {
//     pub struct VideoPlayerWindow(ObjectSubclass<imp::VideoPlayerWindow>)
//         @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
//         @implements gio::ActionMap, gio::ActionGroup;
// }

// impl VideoPlayerWindow {
//     pub fn new<P: IsA<gtk4::Application>>(app: &P) -> Self {
//         glib::Object::builder().property("application", app).build()
//     }

//     fn set_video(&self, video: gio::File) {
//         self.imp().video.set_file(Some(&video));
//     }
// }
pub(crate) fn temp() {

}