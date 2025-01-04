# ironite
Simple static site generator written in Rust.

[My personal blog runs on ironite](https://xiokka.neocities.org/articles/)

## Usage
Create a new project
```bash
ironite new_project $project_name
```
Example
```bash
ironite new_project "My simple static blog"
```
This will generate some basic project files and directories.

projectname.txt => contains project name.

static/base.html => base HTML file that will be used as a template to generate all pages. It contains two placeholdes ($TITLE and $CONTENT) which the generator function will replace accordingly.

static/about.html => The $CONTENT of the homepage. This also contains the placeholder $NAVCLOUD, which the generator function will replace with links to each tag index page (each tag index page contains links to all entries for that tag).

static/style.css => a rather empty CSS file.

entries/ => contains blog entries.

images/ => contains images.

New blog entry
```bash
ironite new_entry $entry_name
```
Example
```bash
ironite new_entry "This is my entry, there are many like it but this one is mine"
```
Once a new entry is created, the corresponding subdirectory is created inside the entries directory. Within that entry, you will find two files: "tags.txt" and "content.html".
tags.txt contains the tags for that entry, where each tag is separated by a whitespace. For example:
```
linux programming command_line
```
assigns that entry the tags "linux", "programming" and "command_line".

content.html is the blog entry itself. The contents of content.html will be inserted into a copy of base.html and thus the blog entry page will be generated.

Generate site
```bash
ironite generate
```
This will create the public/ directory, where the site has been generated.
