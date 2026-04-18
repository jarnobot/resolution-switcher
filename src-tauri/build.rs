fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let icons_dir = std::path::Path::new(&manifest).join("icons");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    build_ico(&icons_dir, &icons_dir.join("icon.ico"));
    build_ico(&icons_dir, &std::path::Path::new(&out_dir).join("icon.ico"));

    tauri_build::build()
}

fn build_ico(icons_dir: &std::path::Path, output: &std::path::Path) {
    let sources = [
        "16x16.png", "32x32.png", "48x48.png",
        "64x64.png", "128x128.png", "128x128@2x.png",
    ];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for name in &sources {
        let path = icons_dir.join(name);
        if let Ok(file) = std::fs::File::open(&path) {
            if let Ok(image) = ico::IconImage::read_png(file) {
                if let Ok(entry) = ico::IconDirEntry::encode(&image) {
                    icon_dir.add_entry(entry);
                }
            }
        }
    }
    if !icon_dir.entries().is_empty() {
        if let Ok(file) = std::fs::File::create(output) {
            let _ = icon_dir.write(file);
        }
    }
}
