use r2d2;
use rusqlite::{self, Connection, Error, OpenFlags};
use std::path::{Path, PathBuf};

enum ConnectionConfig {
    File(PathBuf, OpenFlags),
}

pub struct SqliteConnectionManager {
    config: ConnectionConfig,
}

impl SqliteConnectionManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self::new_with_flags(path, OpenFlags::default())
    }

    pub fn new_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Self {
        SqliteConnectionManager {
            config: ConnectionConfig::File(path.as_ref().to_path_buf(), flags),
        }
    }
}

impl r2d2::ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<Connection, Error> {
        match self.config {
            ConnectionConfig::File(ref path, flags) => Connection::open_with_flags(path, flags),
        }.map_err(Into::into)
    }

    fn is_valid(&self, conn: &mut Connection) -> Result<(), Error> {
        conn.execute_batch("").map_err(Into::into)
    }

    fn has_broken(&self, _: &mut Connection) -> bool {
        false
    }
}
