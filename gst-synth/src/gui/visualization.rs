use gtk::prelude::*;
use gtk::Overlay;
use gtk4 as gtk;
use gtk4::Align;

pub fn visualization(overlay: &Overlay, video_sink: gst::Element) {
    let paintable = video_sink.property::<gtk::gdk::Paintable>("paintable");
    let picture = gtk::Picture::new();
    picture.set_paintable(Some(&paintable));
    picture.set_halign(Align::End);
    picture.set_valign(Align::Start);
    picture.set_margin_top(24);
    picture.set_margin_start(24);
    picture.set_margin_end(24);

    overlay.add_overlay(&picture);
}
