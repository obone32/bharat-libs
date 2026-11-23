//! Embeds the fixture checkout in `../fixture/bharat-fonts` the way an app's build.rs embeds
//! `BHARAT_FONTS_DIR`, so the generated tables are compiled by a real `include!`: once embedded and
//! once served from `OUT_DIR/served`.
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default());
    let fonts = || {
        bharat_fonts_build::Embed::new()
            .dir(root.join("../fixture/bharat-fonts"))
            .required(true)
            .scripts(&["Jpan", "Hant"])
            .filter(|face| face.style == "Regular")
    };
    let embedded = fonts().write("bharat_fonts.rs");
    let served = fonts().serve(out.join("served"), "fonts/").write("bharat_fonts_served.rs");
    if let Err(e) = embedded.and(served) {
        println!("cargo::error={e}");
    }
}
