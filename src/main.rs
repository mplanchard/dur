use std::io;
use std::env;
use std::fs;
use std::path::{PathBuf , Path};


#[derive(Debug)]
enum FSEntry {
    Directory(Directory),
    File(File),
    SymLink(SymLink),
}


#[derive(Debug)]
struct Directory {
    path: PathBuf,
    size: u64,
    dirs: Vec<Directory>,
    files: Vec<File>,
}
impl Directory {
    fn from(path: PathBuf, size: u64) -> Self {
        Self { path, size, dirs: Vec::new(), files: Vec::new() }
    }
    fn populate(&self) -> Self {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        for entry in fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let fs_entry = FSEntry::from(entry.path(), entry.metadata().unwrap());
            match fs_entry {
                FSEntry::File(f) => files.push(f),
                FSEntry::SymLink(_) => {},
                FSEntry::Directory(d) => {
                    dirs.push(d.populate());
                },
            }
        }
        Self { path: self.path.clone(), size: self.size, dirs, files }
    }
}


#[derive(Debug)]
struct File {
    path: PathBuf,
    size: u64,
}
impl File {
    fn from(path: PathBuf, size: u64) -> Self {
        Self { path, size }
    }
}

#[derive(Debug)]
struct SymLink {
    path: PathBuf,
    size: u64,
}
impl SymLink {
    fn from(path: PathBuf, size: u64) -> Self {
        Self { path, size }
    }
}

impl FSEntry {
    fn from(path: PathBuf, metadata: fs::Metadata) -> FSEntry {
        // TODO: handle sad paths (file does not exist, etc.)
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            FSEntry::Directory(Directory::from(path, metadata.len()))
        }
        else if file_type.is_symlink() {
            FSEntry::SymLink(SymLink::from(path, metadata.len()))
        }
        else {
            FSEntry::File(File::from(path, metadata.len()))
        }
    }
}


//**********************************************************************
// Totally naive approach
//**********************************************************************

enum SIPrefixes {
    kB,
    MB,
    GB,
    TB,
}

// todo: continue implementing some kind of automatic byte rollover
// to avoid addition overflow

struct DriveSpace {
    value: u64,
    prefix: SIPrefixes,
}
impl DriveSpace {
    fn add_bytes(&self, byte: u64) {

    }
}


fn _traverse_from_metadata<P: AsRef<Path>> (path: P, metadata: fs::Metadata, levels: u16, mut sum: u64) -> io::Result<(u64)> {
    let file_type = metadata.file_type();
    let mut local_sum = 0;
    if file_type.is_file() {
        let len = metadata.len();
        local_sum += len;
        sum += len;
        println!(
            "{}{} - {}",
            " ".repeat(2 * levels as usize),
            path.as_ref().to_str().unwrap(),
            len,
        );
    }
    else if file_type.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            println!("{}{}/", " ".repeat(2 * levels as usize), path.to_str().unwrap());
            let dirsum = _traverse_from_metadata(path, entry.metadata()?, levels + 1, sum)?;
            println!("Dirsum: {}", dirsum);
            sum += dirsum;
        }
    }
    println!(
        "{}{}/ - {}",
        " ".repeat(2 * levels as usize),
        path.as_ref().to_str().unwrap(),
        local_sum,
    );
    Ok(sum)
}


fn traverse_from_metadata<P: AsRef<Path>> (path: P, metadata: fs::Metadata) -> io::Result<()> {
    let sum = _traverse_from_metadata(path, metadata, 0, 0)?;
    println!("Total: {}", sum);
    Ok(())
}


fn traverse_path<P: AsRef<Path>> (path: &P) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    traverse_from_metadata(path, metadata)?;
    Ok(())
}


fn naive() -> io::Result<()> {
    // Just do it naively for reference.
    for arg in env::args().skip(1).take(1) {
        traverse_path(&arg)?;
    }
    Ok(())
}


fn main() {
    naive();
    // let path = PathBuf::from("/Users/mplanchard/github/rdu/src");
    // let path = PathBuf::from("/Users/mplanchard/github");
    // let metadata = fs::metadata(&path).unwrap();
    // let entry = FSEntry::from(path, metadata);
    // let entry = match entry {
    //     FSEntry::Directory(d) => FSEntry::Directory(d.populate()),
    //     FSEntry::File(f) => FSEntry::File(f),
    //     FSEntry::SymLink(s) => FSEntry::SymLink(s),
    // };

    // println!("{:?}", entry);
}


// fn files_dirs(path: &str) {
//     for _ in fs::read_dir(path)
//         .unwrap()
//         .map(|dir_entry_res| {
//             let dir_entry = dir_entry_res.unwrap();
//             let file_type = dir_entry.file_type().unwrap();
//             if file_type.is_file() {
//                 println!("{:?} - {:?}", dir_entry.path(), size(dir_entry));
//             }
//             else if file_type.is_dir() {
//                 dir_entry.path().to_str().and_then(|path| Some(files_dirs(path)));
//             }
//         }) {};
// }


// fn size(dir_entry: fs::DirFSEntry) -> u64 {
//     dir_entry.metadata().unwrap().len()
// }

// fn main() {
//     println!("Hello, world!");
//     files_dirs(".");
// }


#[cfg(test)]
mod tests {
    #[test]
    fn pass() {
        assert_eq!(1, 1)
    }
}
