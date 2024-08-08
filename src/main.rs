mod generator;
pub use generator::*;

use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

// Basic HTML file structure. Only used when first generation the project. After that we use the base.html contained in the static directory, which may have been modified by the user
const HTML_CONTENT: &str = include_str!("base.html");

// Basic CSS
const CSS_CONTENT: &str = include_str!("style.css");

// Subdirectories created when initializing a new project
const PROJECT_SUBDIRECTORIES:&[&str] = &["entries", "images", "static"];
// ENTRIES: contains blog entries.
// IMAGES: contains all project images.
// STATIC: contains CSS, basic HTML and other elements

fn create_directories(root: &str, sub_dirs: &[&str]) -> io::Result<()> {
    // Create a PathBuf from the root directory
    let root_path = PathBuf::from(root);

    // Iterate over the subdirectories
    for dir in sub_dirs {
        // Create the full path for each subdirectory
        let mut path_buf = root_path.join(dir);
        // Create the directory (and any necessary parent directories)
        fs::create_dir_all(&path_buf)?;
        println!("Created directory: {}", path_buf.display());
    }

    Ok(())
}

fn initialize_project(project_name: &str) -> io::Result<()> {
    // Create project directories
    create_directories(project_name, &PROJECT_SUBDIRECTORIES)?;

    // Path to the projectname.txt file in the root directory
    let projectname_file_path = Path::new(project_name).join("projectname.txt");
    let mut projectname_file = File::create(projectname_file_path)?;
    projectname_file.write_all(project_name.as_bytes())?;

    // Path to the base HTML file within the project directory
    let html_path = Path::new(project_name).join("static").join("base.html");
    let mut base_html = File::create(html_path)?;
    base_html.write_all(HTML_CONTENT.as_bytes())?;


    // Path to the base CSS file within the project directory
    let css_path = Path::new(project_name).join("static").join("style.css");
    let mut base_css = File::create(css_path)?;
    base_css.write_all(CSS_CONTENT.as_bytes())?;


    // Path to the projectname.txt file in the root directory
    let about_file_path = Path::new(project_name).join("static").join("about.html");
    let mut about_file = File::create(about_file_path)?;
    about_file.write_all("<p>This will be shown at the blog index. Edit me at static/about.html</p> $NAVCLOUD".as_bytes())?;

    println!("HTML content saved to static/base.html");
    println!("Project name saved to projectname.txt");

    Ok(())
}


fn initialize_entry(entry_name: &str) -> io::Result<()> {
    // Define the path to the entries directory
    let entries_dir = Path::new("entries");

    // Check if the entries directory exists
    if !entries_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The 'entries' directory does not exist.",
        ));
    }

    // Define the path to the new entry subdirectory
    let entry_path = entries_dir.join(entry_name);

    // Create the new entry subdirectory
    fs::create_dir_all(&entry_path)?;
    println!("Created entry subdirectory: {}", entry_path.display());

    // Define file paths inside the new subdirectory
    let content_file_path = entry_path.join("content.html");
    let tags_file_path = entry_path.join("tags.txt");

    // Create the empty files
    fs::File::create(content_file_path)?;
    fs::File::create(tags_file_path)?;

    println!("Created files: content.html and tags.txt");

    Ok(())
}

fn project_exists() -> bool {
    // Define the path to the projectname.txt file in the current directory
    let path = Path::new("projectname.txt");

    // Use `metadata` to check if the file exists
    path.exists()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No arguments provided.");
        return Ok(());
    }

    let command = &args[1];
    match command.as_str() {
        "new_project" => {
            if args.len() < 3 {
                eprintln!("Error: No project name provided.");
                return Ok(());
            }
            let project_name = &args[2];
            initialize_project(project_name);
        }

        "new_entry" => {
	    if !project_exists() {
		eprintln!("Error: No project found. Please run from project root directory.");
		return Ok(());
	    }
            if args.len() < 3 {
                eprintln!("Error: No entry name provided.");
                return Ok(());
            }
            let entry_name = &args[2];
            initialize_entry(entry_name);
        }

        "print_entries_by_tag" => {
	    let tags_map = filter_entries_by_tag();
    
	    for (tag, paths) in tags_map {
	        println!("Tag: {}", tag);
	        for path in paths {
	            println!("  Path: {}", path.display());
	        }
	    }
        }

        "generate" => {
    		if let Err(e) = generate_site() {
        		eprintln!("Error generating site: {}", e);
    		}
        }

        _ => println!("Unrecognized command: {}", command),
    }

    Ok(())
}
