// use gtk4::{gio, glib, subclass::prelude::*, FileDialog};

// #[derive(Debug, Default, gtk4::CompositeTemplate)]
// #[template(file = "video_player_window.ui")]
// pub struct VideoPlayerWindow {
//     #[template_child(id = "video")]
//     pub video: TemplateChild<gtk4::Video>,
// }

// #[glib::object_subclass]
// impl ObjectSubclass for VideoPlayerWindow {
//     const NAME: &'static str = "VideoPlayerWindow";
//     type Type = super::VideoPlayerWindow;
//     type ParentType = gtk4::ApplicationWindow;

//     fn class_init(klass: &mut Self::Class) {
//         klass.bind_template();
//         klass.install_action_async(
//             "win.open",
//             None,
//             |win, _action_name, _action_target| async move {
//                 let filters = gio::ListStore::new::<gtk4::FileFilter>();
//                 let videos_filter = gtk4::FileFilter::new();
//                 videos_filter.add_mime_type("video/*");
//                 videos_filter.set_name(Some("Video"));
//                 filters.append(&videos_filter);

//                 let audio_filter = gtk4::FileFilter::new();
//                 audio_filter.add_mime_type("audio/*");
//                 audio_filter.set_name(Some("Audio"));
//                 filters.append(&audio_filter);

//                 let dialog = FileDialog::builder()
//                     .title("Open File")
//                     .accept_label("Open")
//                     .modal(true)
//                     .filters(&filters)
//                     .build();

//                 if let Ok(file) = dialog.open_future(Some(&win)).await {
//                     win.set_video(file);
//                 }
//             },
//         );
//     }

//     fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
//         obj.init_template();
//     }
// }

// impl ObjectImpl for VideoPlayerWindow {}
// impl WidgetImpl for VideoPlayerWindow {}
// impl WindowImpl for VideoPlayerWindow {}
// impl ApplicationWindowImpl for VideoPlayerWindow {}
