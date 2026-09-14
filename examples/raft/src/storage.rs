use crate::{Durable, Persistence};
use fs2::FileExt;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

pub struct FileStorage {
    directory: PathBuf,
    _lock: File,
}
impl FileStorage {
    pub fn open(directory: impl AsRef<Path>, expected: Durable) -> io::Result<(Self, Durable)> {
        expected.validate().map_err(io::Error::other)?;
        let directory = directory.as_ref().to_owned();
        fs::create_dir_all(&directory)?;
        // LOCK is also a persistent initialization marker; never remove it on close.
        // create_new distinguishes the sole initializer even when startups race.
        let (lock, first_open) = match OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(directory.join("LOCK"))
        {
            Ok(lock) => (lock, true),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(directory.join("LOCK"))?,
                false,
            ),
            Err(e) => return Err(e),
        };
        lock.try_lock_exclusive()?;
        if first_open {
            // Make the marker durable before initializing any Raft state.
            lock.sync_all()?;
            File::open(&directory)?.sync_all()?;
        }
        let mut storage = Self {
            directory,
            _lock: lock,
        };
        let path = storage.directory.join("state.json");
        let state = match fs::read(&path) {
            Ok(bytes) => {
                let state: Durable = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
                state.validate().map_err(io::Error::other)?;
                if state.cluster != expected.cluster
                    || state.id != expected.id
                    || state.members != expected.members
                {
                    return Err(io::Error::other("storage identity/membership mismatch"));
                }
                state
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if !first_open || storage.directory.join("state.tmp").exists() {
                    return Err(io::Error::other(
                        "missing state in previously opened or incomplete storage; refusing to initialize",
                    ));
                }
                storage.save(&expected)?;
                expected
            }
            Err(e) => return Err(e),
        };
        Ok((storage, state))
    }
}
impl Persistence for FileStorage {
    fn save(&mut self, state: &Durable) -> io::Result<()> {
        let bytes = serde_json::to_vec(state).map_err(io::Error::other)?;
        let mut file = File::create(self.directory.join("state.tmp"))?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        fs::rename(
            self.directory.join("state.tmp"),
            self.directory.join("state.json"),
        )?;
        File::open(&self.directory)?.sync_all()
    }
}
