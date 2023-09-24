use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::configuration_file::ConfigurationFile;
use crate::io::{read_json_from_file, FromFileError};
use crate::types::Directory;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TypescriptProjectReference {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypescriptParentProjectReferenceFile {
    /// This list is expected to be empty, but must be present to satisfy the
    /// TypeScript compiler.
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub references: Vec<TypescriptProjectReference>,
}

#[derive(Debug)]
pub struct TypescriptParentProjectReference {
    directory: Directory,
    pub contents: TypescriptParentProjectReferenceFile,
}

impl ConfigurationFile for TypescriptParentProjectReference {
    type Contents = TypescriptParentProjectReferenceFile;

    const FILENAME: &'static str = "tsconfig.json";

    fn from_directory(
        monorepo_root: &Directory,
        directory: Directory,
    ) -> Result<Self, FromFileError> {
        let filename = monorepo_root.join(&directory).join(Self::FILENAME);
        let manifest_contents: TypescriptParentProjectReferenceFile =
            read_json_from_file(&filename)?;
        Ok(TypescriptParentProjectReference {
            directory,
            contents: manifest_contents,
        })
    }

    fn directory(&self) -> &Directory {
        &self.directory
    }

    fn path(&self) -> PathBuf {
        self.directory.join(Self::FILENAME)
    }

    fn contents(&self) -> &Self::Contents {
        &self.contents
    }
}

#[derive(Debug)]
pub struct TypescriptConfig {
    directory: Directory,
    pub contents: serde_json::Map<String, serde_json::Value>,
}

impl ConfigurationFile for TypescriptConfig {
    type Contents = serde_json::Map<String, serde_json::Value>;

    const FILENAME: &'static str = "tsconfig.json";

    fn from_directory(
        monorepo_root: &Directory,
        directory: Directory,
    ) -> Result<TypescriptConfig, FromFileError> {
        let filename = monorepo_root.join(&directory).join(Self::FILENAME);
        Ok(TypescriptConfig {
            directory,
            contents: read_json_from_file(&filename)?,
        })
    }

    fn directory(&self) -> &Directory {
        &self.directory
    }

    fn path(&self) -> PathBuf {
        self.directory.join(Self::FILENAME)
    }

    fn contents(&self) -> &Self::Contents {
        &self.contents
    }
}
