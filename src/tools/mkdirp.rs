use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Create a new directory, ignore if it already exists.
///
/// Returns the first created directory if some of the
/// paths already existed, `None` if no new directories
/// were created.
pub fn mkdirp<P: AsRef<Path>>(path: P) -> io::Result<Option<PathBuf>> {
    let path = path.as_ref();
    if path == Path::new("") {
        return Ok(None);
    }
    match fs::create_dir(path) {
        Ok(()) => return Ok(Some(path.to_owned())),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => return Ok(None),
        Err(_) if path.is_dir() => return Ok(None),
        Err(e) => return Err(e),
    }
    let created = match path.parent() {
        Some(p) => mkdirp(p),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to create whole tree",
            ))
        }
    };
    match fs::create_dir(path) {
        Ok(()) => created,
        Err(_) if path.is_dir() => created,
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => created,
        Err(e) => Err(e),
    }
}
