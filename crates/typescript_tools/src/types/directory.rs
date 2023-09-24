use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// A non-empty path terminating in a directory.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Directory(PathBuf);

impl Directory {
    pub fn unchecked_from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self(path.as_ref().to_path_buf())
    }
}

impl AsRef<Path> for Directory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Deref for Directory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.0.as_path()
    }
}
