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
"#,
    );
    provider
}
