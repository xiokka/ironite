use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::io::prelude::*;

// Read file content into a String
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

// Replace placeholders in HTML content
pub fn replace_placeholders(html_content: &str, placeholders: &HashMap<String, String>) -> String {
    let mut result = html_content.to_string();
    for (placeholder, replacement) in placeholders {
        result = result.replace(placeholder, replacement);
    }
    result
}

// Write HTML content to a file
pub fn write_html_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content.as_bytes())
}

// Create directories if they don't exist
fn create_directories<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)
    } else {
        Ok(())
    }
}

// Convert text to HTML
pub fn txt_to_html(content: Vec<u8>) -> Vec<u8> {
    let text = String::from_utf8_lossy(&content);
    let html_content = format!("<p>{}</p>", text.replace("\n", "</p><p>"));
    html_content.into_bytes()
}

// Get unique tags from a tags file
pub fn get_tags(tags_file_path: &str) -> HashSet<String> {
    let content = match fs::read_to_string(tags_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading tags file {}: {}", tags_file_path, e);
            return HashSet::new();
        }
    };

    let mut unique_tags = HashSet::new();
    for tag in content.split_whitespace() {
        let trimmed = tag.trim();
        if !trimmed.is_empty() {
            unique_tags.insert(trimmed.to_string());
        }
    }

    unique_tags
}

// Filter entries by tags
pub fn filter_entries_by_tag() -> HashMap<String, Vec<PathBuf>> {
    let entries_dir = Path::new("entries");
    let mut tags_map = HashMap::new();

    if !entries_dir.exists() || !entries_dir.is_dir() {
        eprintln!("Entries directory does not exist or is not a directory.");
        return tags_map;
    }

    if let Ok(entries) = fs::read_dir(entries_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let tags_file_path = path.join("tags.txt");
            let tags = get_tags(tags_file_path.to_str().unwrap_or(""));

            for tag in tags {
                tags_map.entry(tag)
                    .or_insert_with(Vec::new)
                    .push(path.clone());
            }
        }
    } else {
        eprintln!("Failed to read the entries directory.");
    }

    tags_map
}

// Copy directory and its contents recursively
pub fn copy_directory<P: AsRef<Path>>(source: P, destination: P) -> io::Result<()> {
    let source = source.as_ref();
    let destination = destination.as_ref();

    if !source.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Source is not a directory"));
    }

    // Create destination directory if it does not exist
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(source).unwrap(); // Path relative to source

        let new_destination = destination.join(relative_path);

        if path.is_dir() {
            copy_directory(path, new_destination)?;
        } else {
            fs::copy(path, new_destination)?;
        }
    }

    Ok(())
}

// Generate pages for entries
pub fn generate_entry_pages(base_html: &str, entries_dir: &Path, public_entries_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(entries_dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        if !entry_path.is_dir() {
            continue;
        }

        let title = entry_path.file_name().unwrap().to_str().unwrap_or("Untitled");
        let new_entry_dir = public_entries_dir.join(title);
        create_directories(&new_entry_dir)?;

        let content_file_path = entry_path.join("content.html");
        if content_file_path.exists() {
            let content_html = fs::read(&content_file_path)?;
            //let content_html = txt_to_html(content);
            let final_html_content = replace_placeholders(
                &base_html,
                &[
                    ("$CONTENT".to_string(), String::from_utf8_lossy(&content_html).to_string()),
                    ("$TITLE".to_string(), title.to_string()),
		    ("$NAVCLOUD".to_string(), "".to_string()),
                ].iter().cloned().collect()
            );
            write_html_file(new_entry_dir.join("index.html"), &final_html_content)?;
        } else {
            eprintln!("No content.html found in {:?}", entry_path);
        }
    }
    Ok(())
}

pub fn generate_tag_pages(base_html: &str, tags_map: &HashMap<String, Vec<PathBuf>>, public_dir: &Path) -> io::Result<()> {
    // Collect tags and sort them
    let mut tags: Vec<String> = tags_map.keys().cloned().collect();
    tags.sort();

    // Iterate over sorted tags
    for tag in tags {
        if let Some(paths) = tags_map.get(&tag) {
            let tag_dir = public_dir.join(&tag);
            create_directories(&tag_dir)?;

            let mut tag_content = String::new();
            for path in paths {
                let entry_title = path.file_name().unwrap().to_str().unwrap_or("Untitled");
                let entry_link = format!("<a href=\"../entries/{}/index.html\">{}</a><br>", entry_title, entry_title);
                tag_content.push_str(&entry_link);
            }

            let tag_html_content = replace_placeholders(
                &base_html,
                &[
                    ("$CONTENT".to_string(), tag_content),
                    ("$TITLE".to_string(), tag.to_string()),
                    ("$NAVCLOUD".to_string(), "".to_string()),
                ].iter().cloned().collect()
            );
            write_html_file(tag_dir.join("index.html"), &tag_html_content)?;
        }
    }
    Ok(())
}

// Generate the site
pub fn generate_site() -> io::Result<()> {
    let public_dir = Path::new("public");
    let entries_dir = public_dir.join("entries");
    let root_entries_dir = Path::new("entries");
    let base_html_path = Path::new("static").join("base.html");
    let about_txt_path = Path::new("static").join("about.html");
    let projectname_path = Path::new("projectname.txt");

    // Read base HTML
    let base_html = match read_file_to_string(&base_html_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read base HTML file: {}", e);
            return Err(e);
        }
    };

    // Read other static content
    let about_txt_content = match read_file_to_string(&about_txt_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read about.html: {}", e);
            return Err(e);
        }
    };

    let project_name = match read_file_to_string(&projectname_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read projectname.txt: {}", e);
            return Err(e);
        }
    };

    // Create public directories
    create_directories(&public_dir)?;
    create_directories(&entries_dir)?;

    // Copy static files
    let static_source = Path::new("static");
    let static_dest = public_dir.join("static");
    copy_directory(static_source, &static_dest)?;

    // Copy images files
    let static_source = Path::new("images");
    let static_dest = public_dir.join("images");
    copy_directory(static_source, &static_dest)?;

    // Generate entry pages
    generate_entry_pages(&base_html, &root_entries_dir, &entries_dir)?;

    // Generate tag pages
    let tags_map = filter_entries_by_tag();
    generate_tag_pages(&base_html, &tags_map, &public_dir)?;

    // Create navigation cloud. Contains links to each tag index
    let mut tags: Vec<String> = tags_map.keys().cloned().collect();
    tags.sort();

    let nav_cloud = tags
        .into_iter()
        .map(|tag| format!("<a href=\"{}/index.html\">{}</a>", tag, tag))
        .collect::<String>();


    // Replace the $NAVCLOUD placeholder in about_txt_content with tags
    let parsed_about_txt_content = replace_placeholders(
	&about_txt_content,
	&[
	    ("$NAVCLOUD".to_string(), nav_cloud),
        ].iter().cloned().collect()
    );
    // Generate the root index.html
    let root_index_html_content = replace_placeholders(
        &base_html,
        &[
            ("$CONTENT".to_string(), parsed_about_txt_content),
            ("$TITLE".to_string(), project_name),
            ("$NAVCLOUD".to_string(), "".to_string()),
        ].iter().cloned().collect()
    );
    write_html_file(public_dir.join("index.html"), &root_index_html_content)?;


// Generate entries index.html
let mut entries: Vec<_> = fs::read_dir(entries_dir.clone())?
    .filter_map(Result::ok)  // Filter out any errors
    .filter(|entry| entry.path().is_dir())  // Only include directories
    .collect();

// Sort the entries by their titles
entries.sort_by(|a, b| {
    a.path().file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("Untitled"))
        .cmp(&b.path().file_name().unwrap_or_else(|| std::ffi::OsStr::new("Untitled")))
});

let mut entries_index_content = String::new();
for entry in entries {
    let entry_path = entry.path();
    let title = entry_path.file_name().unwrap().to_str().unwrap_or("Untitled");
    let entry_link = format!("<a href=\"{}/index.html\">{}</a><br>", title, title);
    entries_index_content.push_str(&entry_link);
}


    let entries_index_html_content = replace_placeholders(
        &base_html,
        &[
            ("$CONTENT".to_string(), entries_index_content),
            ("$TITLE".to_string(), "Entries".to_string()),
	    ("$NAVCLOUD".to_string(), "".to_string()),
        ].iter().cloned().collect()
    );
    write_html_file(entries_dir.join("index.html"), &entries_index_html_content)?;

    Ok(())
}
