use gstreamer::prelude::ElementExt;

pub(crate) fn stop(element: &gstreamer::Element) {
    element.set_state(gstreamer::State::Null);
}
