use std::path::{PathBuf, Path};
use thiserror::Error;
use tokio::{fs::{self, File, try_exists}, io::AsyncWriteExt};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Error)]
pub enum QmdlStoreError {
    #[error("Can't close an entry when there's no current entry")]
    NoCurrentEntry,
    #[error("Couldn't create file: {0}")]
    CreateFileError(tokio::io::Error),
    #[error("Couldn't read file: {0}")]
    ReadFileError(tokio::io::Error),
    #[error("Couldn't open directory at path: {0}")]
    OpenDirError(tokio::io::Error),
    #[error("Couldn't read manifest file: {0}")]
    ReadManifestError(tokio::io::Error),
    #[error("Couldn't write manifest file: {0}")]
    WriteManifestError(tokio::io::Error),
    #[error("Couldn't parse QMDL store manifest file: {0}")]
    ParseManifestError(toml::de::Error)
}

pub struct QmdlStore {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub current_entry: Option<usize>, // index into manifest
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Manifest {
    pub entries: Vec<ManifestEntry>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct ManifestEntry {
    pub name: String,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub size_bytes: usize,
}

impl ManifestEntry {
    fn new() -> Self {
        let now = Local::now();
        ManifestEntry {
            name: format!("{}", now.timestamp()),
            start_time: now,
            end_time: None,
            size_bytes: 0,
        }
    }
}

impl QmdlStore {
    // Returns whether a directory with a "manifest.toml" exists at the given
    // path (though doesn't check if that manifest is valid)
    pub async fn exists<P>(path: P) -> Result<bool, QmdlStoreError> where P: AsRef<Path> {
        let manifest_path = path.as_ref().join("manifest.toml");
        let dir_exists = try_exists(path).await.map_err(QmdlStoreError::OpenDirError)?;
        let manifest_exists = try_exists(manifest_path).await.map_err(QmdlStoreError::ReadManifestError)?;
        Ok(dir_exists && manifest_exists)
    }

    // Loads an existing QmdlStore at the given path. Errors if no store exists,
    // or if it's malformed.
    pub async fn load<P>(path: P) -> Result<Self, QmdlStoreError> where P: AsRef<Path> {
        let path: PathBuf = path.as_ref().to_path_buf();
        let manifest = QmdlStore::read_manifest(&path).await?;
        Ok(QmdlStore {
            path,
            manifest,
            current_entry: None,
        })
    }

    // Creates a new QmdlStore at the given path. This involves creating a dir
    // and writing an empty manifest.
    pub async fn create<P>(path: P) -> Result<Self, QmdlStoreError> where P: AsRef<Path> {
        let manifest_path = path.as_ref().join("manifest.toml");
        fs::create_dir_all(&path).await
            .map_err(QmdlStoreError::OpenDirError)?;
        let mut manifest_file = File::create(&manifest_path).await
            .map_err(QmdlStoreError::WriteManifestError)?;
        let empty_manifest = Manifest { entries: Vec::new() };
        let empty_manifest_contents = toml::to_string_pretty(&empty_manifest)
            .expect("failed to serialize manifest");
        manifest_file.write_all(empty_manifest_contents.as_bytes()).await
            .map_err(QmdlStoreError::WriteManifestError)?;
        QmdlStore::load(path).await
    }

    async fn read_manifest<P>(path: P) -> Result<Manifest, QmdlStoreError> where P: AsRef<Path> {
        let manifest_path = path.as_ref().join("manifest.toml");
        let file_contents = fs::read_to_string(&manifest_path).await
            .map_err(QmdlStoreError::ReadManifestError)?;
        toml::from_str(&file_contents)
            .map_err(QmdlStoreError::ParseManifestError)
    }

    // Closes the current entry (if needed), creates a new entry based on the
    // current time, and updates the manifest
    pub async fn new_entry(&mut self) -> Result<File, QmdlStoreError> {
        // if we've already got an entry open, close it
        if self.current_entry.is_some() {
            self.close_current_entry().await?;
        }
        let new_entry = ManifestEntry::new();
        let mut file_path = self.path.join(&new_entry.name);
        file_path.set_extension("qmdl");
        let file = File::options()
            .create(true)
            .write(true)
            .open(&file_path).await
            .map_err(QmdlStoreError::CreateFileError)?;
        self.manifest.entries.push(new_entry);
        self.current_entry = Some(self.manifest.entries.len() - 1);
        self.write_manifest().await?;
        Ok(file)
    }

    pub async fn open_entry(&self, entry: &ManifestEntry) -> Result<File, QmdlStoreError> {
        let mut file_path = self.path.join(&entry.name);
        file_path.set_extension("qmdl");
        File::open(file_path).await
            .map_err(QmdlStoreError::ReadFileError)
    }

    // Sets the current entry's end_time, updates the manifest, and unsets the
    // current entry
    pub async fn close_current_entry(&mut self) -> Result<(), QmdlStoreError> {
        let entry_index = self.current_entry.take()
            .ok_or(QmdlStoreError::NoCurrentEntry)?;
        self.manifest.entries[entry_index].end_time = Some(Local::now());
        self.write_manifest().await
    }

    // Sets the given entry's size, updating the manifest
    pub async fn update_entry_size(&mut self, entry_index: usize, size_bytes: usize) -> Result<(), QmdlStoreError> {
        self.manifest.entries[entry_index].size_bytes = size_bytes;
        self.write_manifest().await
    }

    async fn write_manifest(&mut self) -> Result<(), QmdlStoreError> {
        let mut manifest_file = File::options()
            .write(true)
            .open(self.path.join("manifest.toml")).await
            .map_err(QmdlStoreError::WriteManifestError)?;
        let manifest_contents = toml::to_string_pretty(&self.manifest)
            .expect("failed to serialize manifest");
        manifest_file.write_all(manifest_contents.as_bytes()).await
            .map_err(QmdlStoreError::WriteManifestError)?;
        Ok(())
    }

    // Finds an entry by filename
    pub fn entry_for_name(&self, name: &str) -> Option<ManifestEntry> {
        self.manifest.entries.iter()
            .find(|entry| entry.name == name)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use super::*;

    #[tokio::test]
    async fn test_load_from_empty_dir() {
        let dir = TempDir::new("qmdl_store_test").unwrap();
        assert!(!QmdlStore::exists(dir.path()).await.unwrap());
        let _created_store = QmdlStore::create(dir.path()).await.unwrap();
        assert!(QmdlStore::exists(dir.path()).await.unwrap());
        let loaded_store = QmdlStore::load(dir.path()).await.unwrap();
        assert_eq!(loaded_store.manifest.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_creating_updating_and_closing_entries() {
        let dir = TempDir::new("qmdl_store_test").unwrap();
        let mut store = QmdlStore::create(dir.path()).await.unwrap();
        let _ = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        assert_eq!(QmdlStore::read_manifest(dir.path()).await.unwrap(), store.manifest);

        store.update_entry_size(entry_index, 1000).await.unwrap();
        assert_eq!(store.manifest.entries[entry_index].size_bytes, 1000);
        assert_eq!(QmdlStore::read_manifest(dir.path()).await.unwrap(), store.manifest);

        assert!(store.manifest.entries[entry_index].end_time.is_none());
        store.close_current_entry().await.unwrap();
        let entry = store.entry_for_name(&store.manifest.entries[entry_index].name).unwrap();
        assert!(entry.end_time.is_some());
        assert_eq!(QmdlStore::read_manifest(dir.path()).await.unwrap(), store.manifest);

        assert!(matches!(store.close_current_entry().await, Err(QmdlStoreError::NoCurrentEntry)));
    }

    #[tokio::test]
    async fn test_repeated_new_entries() {
        let dir = TempDir::new("qmdl_store_test").unwrap();
        let mut store = QmdlStore::create(dir.path()).await.unwrap();
        let _ = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        let _ = store.new_entry().await.unwrap();
        let new_entry_index = store.current_entry.unwrap();
        assert_ne!(entry_index, new_entry_index);
        assert_eq!(store.manifest.entries.len(), 2);
    }
}
