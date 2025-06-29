use std::{path::Path};
use markdown;

use walkdir::WalkDir;

const CONTENT_DIR: &str = "content/";
const SITE_DIR: &str = "site/";
const OUTPUT_DIR: &str = "target/site";
const BODY_TEMPLATE_VAR: &str = "{{ body }}";

fn main() {
    for entry in WalkDir::new(SITE_DIR).into_iter().filter_map(|e| e.ok()) {
        if entry.path() == Path::new(SITE_DIR) {
            continue;
        }
        let base_html = std::fs::read_to_string(entry.path())
            .expect("Failed to read base HTML file");

        let base_file_name = String::from(entry.path().to_str().expect("Failed to convert path to str"));
        let base_file_name = base_file_name.strip_prefix(SITE_DIR)
            .expect("Failed to strip content directory prefix")
            .strip_suffix(".html")
            .expect("Failed to strip .html suffix");
        let content_file_name = format!("{}{}.md", CONTENT_DIR, base_file_name);

        let full_html = if std::fs::exists(Path::new(&content_file_name)).unwrap() {
            let content = std::fs::read_to_string(&content_file_name)
                .expect("Failed to read markdown file");
            let rendered_content = markdown::to_html(&content);
            base_html.replace(BODY_TEMPLATE_VAR, &rendered_content)
        } else {
            base_html
        };

        let suffix = if entry.file_name().to_str() == Some("index.html") {
            ".html"
        } else {
            ""
        };

        let output_path = Path::new(OUTPUT_DIR).join(format!("{}{}", base_file_name, suffix));
        println!("Converting {} to {}", entry.path().display(), output_path.display());
        std::fs::create_dir_all(output_path.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_path, full_html)
            .expect("Failed to write full HTML file");
    }
}
