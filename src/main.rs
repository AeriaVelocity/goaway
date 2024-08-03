// Go Away - a GTK4 utility which provides unhelpful information on files,
// which makes you want to tell it to "go away".

use gtk4 as gtk;
use gtk::prelude::*;

use std::fmt::Display;

struct File {
    path: String,
    contents: String,
    size: u64,
    is_dir: bool,
    is_executable: bool,
    is_hidden: bool,
    is_symlink: bool,
}

struct Dir {
    path: String,
    contents: Vec<File>,
}

impl File {
    fn new(filename: &str, contents: &str) -> Self {
        Self {
            path: format!("./{}", filename),
            contents: String::from(contents),
            size: contents.len() as u64,
            is_dir: false,
            is_executable: false,
            is_hidden: false,
            is_symlink: false,
        }
    }
    fn new_dir() -> Self {
        Self {
            path: String::from("./"),
            contents: String::from(""),
            size: 0,
            is_dir: true,
            is_executable: false,
            is_hidden: false,
            is_symlink: false,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_dir {
            writeln!(f, "This file is a directory.")?;
            writeln!(f, "Directory: {}", self.path)?;
            writeln!(f, "Is hidden: {}", self.is_hidden)?;
            writeln!(f, "Is symlink: {}", self.is_symlink)?;
            return Ok(());
        }
        writeln!(f, "File: {}", self.path)?;
        writeln!(f, "Size: {}", self.size)?;
        writeln!(f, "Is executable: {}", self.is_executable)?;
        writeln!(f, "Is hidden: {}", self.is_hidden)?;
        writeln!(f, "Is symlink: {}", self.is_symlink)?;
        Ok(())
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dir: {}\n", self.path)?;
        writeln!(f, "Contents")?;
        writeln!(f, "---------")?;
        for file in &self.contents {
            writeln!(f, "{}", file)?;
        }
        writeln!(f, "---------")?;
        Ok(())
    }
}

fn main() {
    let app = gtk::Application::new(Some("org.aeriavelocity.goaway"), Default::default());
    app.connect_activate(|app| activate(app));
    app.run();
}

fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title(Some("Go Away"));
    window.set_default_size(640, 480);

    main_app(&window);

    window.show();
}

fn main_app(window: &gtk::ApplicationWindow) {
    let hello_world_file: File = File::new("hello_world.txt", "Hello World");
    let dir: Dir = Dir {
        path: String::from("./"),
        contents: vec![
            hello_world_file,
            File::new_dir(),
        ],
    };
    let dir_as_label = gtk::Label::new(Some(&format!("{}", dir)));
    window.set_child(Some(&dir_as_label));
}
