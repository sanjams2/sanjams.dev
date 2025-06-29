use std::path::{Path, PathBuf};
use markdown;

use walkdir::WalkDir;

const CONTENT_DIR: &str = "content/";
const SITE_DIR: &str = "site/";
const HTML_TEMPLATE_FILE_PATH: &str = "site/template.html";
const OUTPUT_DIR: &str = "target/site/";
const BODY_TEMPLATE_VAR: &str = "{{ body }}";

fn main() {
    let template_file = std::fs::read_to_string(HTML_TEMPLATE_FILE_PATH).expect("Failed to read HTML template file");

    for entry in WalkDir::new(CONTENT_DIR).into_iter().filter_map(|e| e.ok()) {
        if entry.path() == Path::new(CONTENT_DIR) {
            continue;
        }
        let base_file_name = String::from(entry.path().to_str().expect("Failed to convert path to str"));
        let base_file_name = base_file_name.strip_prefix(CONTENT_DIR)
            .expect("Failed to strip content directory prefix")
            .strip_suffix(".md")
            .expect("Failed to strip .html suffix");

        let html_name_suffix = if entry.file_name().to_str() == Some("index.md") {
            ".html"
        } else {
            ""
        };
        let html_file = PathBuf::from(format!("{}{}.html", SITE_DIR, base_file_name));
        let content_file = PathBuf::from(format!("{}{}.md", CONTENT_DIR, base_file_name));
        let output_file = PathBuf::from(format!("{}{}{}", OUTPUT_DIR, base_file_name, html_name_suffix));

        let base_html = if html_file.exists() {
            std::fs::read_to_string(&html_file).expect("Failed to read base HTML file")
        } else {
            template_file.clone()
        };
        let content = std::fs::read_to_string(&content_file)
            .expect("Failed to read markdown file");
        let rendered_content = markdown::to_html(&content);
        let full_html = base_html.replace(BODY_TEMPLATE_VAR, &rendered_content);

        println!("Converting {} to {}", entry.path().display(), output_file.display());
        std::fs::create_dir_all(output_file.parent().unwrap())
            .expect("Failed to create output directory");
        std::fs::write(output_file, full_html)
            .expect("Failed to write full HTML file");
    }
}
