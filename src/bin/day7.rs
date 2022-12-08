use color_eyre::Result;
use parking_lot::Mutex;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

struct File {
    name: String,
    size: usize,
}

trait ByteCount {
    fn bytes(&self) -> usize;
}

impl ByteCount for File {
    fn bytes(&self) -> usize {
        self.size
    }
}

struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: Vec<Arc<Mutex<Directory>>>,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            directories: Vec::new(),
        }
    }

    /// Add a file to the directory.  Does _not_ overwrite files.
    fn add_file(&mut self, name: String, size: usize) {
        self.files
            .entry(name.clone())
            .or_insert(File { name, size });
    }

    fn add_directory(&mut self, name: String) -> Arc<Mutex<Directory>> {
        let dir = Directory::new(name);
        let new_dir = Arc::new(Mutex::new(dir));
        self.directories.push(new_dir.clone());
        new_dir
    }

    fn get_or_add_directory(&mut self, name: String) -> Arc<Mutex<Directory>> {
        {
            let mut iter = self.directories.iter();
            if let Some(d) = iter.find(|d| d.lock().name == name) {
                return d.clone();
            }
        }
        self.add_directory(name)
    }

    fn walk_children(&self) -> Vec<Arc<Mutex<Directory>>> {
        let mut vec: Vec<_> = self.directories.iter().cloned().collect();
        for directory in &self.directories {
            vec.append(&mut directory.lock().walk_children());
        }
        vec
    }
}

impl ByteCount for Directory {
    fn bytes(&self) -> usize {
        self.files.values().map(ByteCount::bytes).sum::<usize>()
            + self
                .directories
                .iter()
                .map(|d| d.lock().bytes())
                .sum::<usize>()
    }
}

fn main() -> Result<()> {
    let mut iter = include_str!("../../day7.txt").lines();

    let root = Arc::new(Mutex::new(Directory::new("/".to_owned())));

    let filesystem_space = 70_000_000usize;
    let needed_space = 30_000_000usize;

    let mut current_dir = root.clone();
    let mut parent_dir_stack = Vec::new();
    // Read the directory tree and build it.
    while let Some(line) = iter.next() {
        let mut tokens = line.split(' ');
        match tokens.next() {
            // A Command is happening
            Some("$") => {
                match match tokens.next().unwrap() {
                    // Nothing to do, we'll parse the files as they come on subsequent lines.
                    "ls" => continue,
                    "cd" => tokens.next().unwrap(),
                    unkn => panic!("Unknown commmand {unkn} encountered"),
                } {
                    "/" => {
                        current_dir = root.clone();
                        parent_dir_stack = Vec::new();
                    }
                    ".." => current_dir = parent_dir_stack.pop().unwrap(),
                    dirname => {
                        let new_dir = current_dir.lock().get_or_add_directory(dirname.to_owned());
                        let old_current = std::mem::replace(&mut current_dir, new_dir);
                        parent_dir_stack.push(old_current);
                    }
                }
            }
            // A directory listing
            Some("dir") => {
                let dirname = tokens.next().unwrap();
                current_dir.lock().add_directory(dirname.to_owned());
            }
            // A file listing
            Some(nums) => {
                let filename = tokens.next().unwrap();
                current_dir
                    .lock()
                    .add_file(filename.to_owned(), nums.parse::<usize>().unwrap());
            }
            None => panic!("expected a token on the line"),
        }
    }

    let under_onek = root
        .lock()
        .walk_children()
        .iter()
        .filter(|d| d.lock().bytes() < 100_000)
        .map(|d| d.lock().bytes())
        .sum::<usize>();
    println!("Total of all under 100,000: {under_onek}");

    let total_taken_space = root.lock().bytes();
    let to_delete_space = needed_space - (filesystem_space - total_taken_space);

    println!("Total filesystem: {total_taken_space} we need to free at laest {to_delete_space}.");

    let (to_delete, size) = root
        .lock()
        .walk_children()
        .iter()
        .map(|d| (d.clone(), d.lock().bytes()))
        .filter(|(_d, bytes)| *bytes > to_delete_space)
        .min_by_key(|(_d, bytes)| bytes - to_delete_space)
        .unwrap();

    println!(
        "The directory {} contains {size} bytes, we can delete it",
        to_delete.lock().name
    );

    Ok(())
}
