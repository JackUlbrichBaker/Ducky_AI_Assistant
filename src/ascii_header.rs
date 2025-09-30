use figlet_rs::FIGfont;

/// Renders a text string into an ASCII-art header
pub fn render_ascii_header(text: &str) -> String {
    // Standard font bundled with figlet-rs
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(text);

    match figure {
        Some(fig) => fig.to_string(),
        None => text.to_string(),
    }
}

