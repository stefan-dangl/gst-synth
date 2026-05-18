use gtk4::CssProvider;

pub fn style() -> CssProvider {
    let provider = CssProvider::new();
    provider.load_from_data(
        r#"
frame.white-key {
    background: #f5f5f5;
    border: none;
    box-shadow: none;
}

frame.white-key:hover {
    background: #d8d8d8;
}

frame.black-key {
    background: #222;
    border: none;
    box-shadow: none;
}

frame.black-key:hover {
    background: #3a3a3a;
}

frame.black-key.selected {
    background: #4a3800;
}

frame.black-key.selected:hover {
    background: #5c4600;
}

label.white-key-label {
    color: #111;
}

label.black-key-label {
    color: #fff;
}

label.octave-value {
    color: #fff;
    font-size: 24px;
    font-weight: bold;
}

label.knob-label {
    color: #aaa;
    font-size: 12px;
}

label.knob-value {
    color: #ddd;
    font-size: 12px;
}

frame.effect-section {
    background-color: rgba(30, 30, 30, 0.85);
    border-radius: 4px;
}

frame.effect-section > label {
    color: #fff;
    font-size: 11px;
    font-weight: bold;
}
"#,
    );
    provider
}
