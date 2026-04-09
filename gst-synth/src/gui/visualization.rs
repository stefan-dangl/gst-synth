use crate::types::WaveForm;
use crate::types::{Command, Note};
use gtk::GestureClick;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Frame, Label, Orientation, Overlay};
use gtk4 as gtk;
use gtk4::Align;

pub fn visualization(overlay: &Overlay, video_sink: gst::Element) {
    let video_widget = video_sink.property::<gtk::Widget>("widget");
    video_widget.set_halign(Align::End);
    video_widget.set_valign(Align::Start);
    video_widget.set_margin_top(24);
    video_widget.set_margin_start(24);
    video_widget.set_margin_end(24);

    overlay.add_overlay(&video_widget);
}
