use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;

pub fn get_entries(root: & String, recur: bool) -> io::Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = Vec::new();
    let path = Path::new(root);
    match recur {
        true => try!(walk(path, &mut entries)),
        false => try!(explore(path, &mut entries)),
    }
    Ok(entries)
}

fn walk(dir: & Path, entries: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(walk(&path, entries));
            } else {
                entries.push(entry);
            }
        }
    }
    Ok(())
}

fn explore(dir: & Path, entries: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if !path.is_dir() {
                entries.push(entry);
            }
        }
    }
    Ok(())
}
