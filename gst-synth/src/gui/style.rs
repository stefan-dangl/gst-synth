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

frame.black-key {
    background: #222;
    border: none;
    box-shadow: none;
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
    font-size: 10px;
}

label.knob-value {
    color: #ddd;
    font-size: 10px;
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
