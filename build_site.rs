use std::{ffi::OsStr, path::Path};
use markdown;

use walkdir::WalkDir;

const CONTENT_DIR: &str = "content/";
const OUTPUT_DIR: &str = "target/site";
const BODY_TEMPLATE_VAR: &str = "{{ body }}";

fn main() {
    for entry in WalkDir::new(CONTENT_DIR).into_iter().filter_map(|e| e.ok()) {
        if entry.path() == Path::new(CONTENT_DIR) || entry.path().extension() == Some(&OsStr::new(".md")) {
            continue;
        }
        let body = std::fs::read_to_string(entry.path())
            .expect("Failed to read markdown file");
        let rendered_body = markdown::to_html(&body);

        let file_name = String::from(entry.path().to_str().expect("Failed to convert path to str"));
        let file_name = file_name
            .strip_prefix(CONTENT_DIR)
            .expect("Failed to strip content directory prefix")
            .replace(".md", "");

        let base_html = std::fs::read_to_string(Path::new(&format!("site/{}.html", file_name)))
            .expect("Failed to read base HTML file");
        let full_html = base_html.replace(BODY_TEMPLATE_VAR, &rendered_body);
        let output_path = Path::new(OUTPUT_DIR).join(format!("{}.html", file_name));
        println!("Converting {} to {}/{}.html", entry.path().display(), OUTPUT_DIR, file_name);
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, full_html)
            .expect("Failed to write full HTML file");
    }
}
