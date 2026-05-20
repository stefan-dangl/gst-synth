use gtk::prelude::*;
use gtk4 as gtk;

pub fn visualization(video_sink: gst::Element) -> gtk::Picture {
    let paintable = video_sink.property::<gtk::gdk::Paintable>("paintable");
    let picture = gtk::Picture::new();
    picture.set_paintable(Some(&paintable));
    picture.set_vexpand(false);
    picture
}
