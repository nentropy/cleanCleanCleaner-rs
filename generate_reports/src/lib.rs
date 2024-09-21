/// # Utility functions for use in blackhat Rust
/// 1. Report Generator
/// ### Uses handlebars and a markdown generator to generate reports
/// ### Each is timestampted and named after the title.
/// 
mod ai_completions;
use chrono::prelude::*;
use std::fs::File;
use std::io::Write;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ai_completions::get_api_keys;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Section {
    title: String,
    content: String,
    subsections: Vec<Section>,
}

#[derive(Serialize, Deserialize)]
struct TemplateData {
    title: String,
    sections: Vec<Section>,
}

// CALL AI MODEL

fn generate_markdown(data: &TemplateData) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);

    let template = r#"
# {{title}}

{{#each sections}}
{{#if title}}## {{title}}{{/if}}

{{content}}

{{#each subsections}}
{{#if title}}### {{title}}{{/if}}

{{content}}

{{#each subsections}}
{{#if title}}#### {{title}}{{/if}}

{{content}}
{{/each}}
{{/each}}
{{/each}}
"#;

    handlebars.register_template_string("markdown", template)?;
    Ok(handlebars.render("markdown", &data)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = TemplateData {
        title: "My Dynamic Markdown Document".to_string(),
        sections: vec![
            Section {
                title: "Introduction".to_string(),
                content: "This is the introduction.".to_string(),
                subsections: vec![],
            },
            Section {
                title: "Main Content".to_string(),
                content: "This is the main content.".to_string(),
                subsections: vec![
                    Section {
                        title: "Subsection 1".to_string(),
                        content: "This is subsection 1.".to_string(),
                        subsections: vec![
                            Section {
                                title: "Sub-subsection".to_string(),
                                content: "This is a sub-subsection.".to_string(),
                                subsections: vec![],
                            },
                        ],
                    },
                    Section {
                        title: "Subsection 2".to_string(),
                        content: "This is subsection 2.".to_string(),
                        subsections: vec![],
                    },
                ],
            },
            Section {
                title: "Conclusion".to_string(),
                content: "This is the conclusion.".to_string(),
                subsections: vec![],
            },
        ],
    };

    let markdown = generate_markdown(&data)?;
    println!("{}", markdown);
    let file_name: String = "../reports/{}_{}.md", chrono::Utc::now().format("%Y-%m-%d"), {data.title};
    // Optionally, save to a file
    fs::write(&file_name, markdown)?;

    Ok(())
}
