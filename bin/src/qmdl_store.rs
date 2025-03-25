use rayhunter::util::RuntimeMetadata;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::{
    fs::{self, try_exists, File, OpenOptions},
    io::AsyncWriteExt,
};

#[derive(Debug, Error)]
pub enum RecordingStoreError {
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
    ParseManifestError(toml::de::Error),
}

pub struct RecordingStore {
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
    pub last_message_time: Option<DateTime<Local>>,
    pub qmdl_size_bytes: usize,
    pub analysis_size_bytes: usize,
    pub rayhunter_version: Option<String>,
    pub system_os: Option<String>,
    pub arch: Option<String>,
}

impl ManifestEntry {
    fn new() -> Self {
        let now = Local::now();
        let metadata = RuntimeMetadata::new();
        ManifestEntry {
            name: format!("{}", now.timestamp()),
            start_time: now,
            last_message_time: None,
            qmdl_size_bytes: 0,
            analysis_size_bytes: 0,
            rayhunter_version: Some(metadata.rayhunter_version),
            system_os: Some(metadata.system_os),
            arch: Some(metadata.arch),
        }
    }

    pub fn get_qmdl_filepath<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let mut filepath = path.as_ref().join(&self.name);
        filepath.set_extension("qmdl");
        filepath
    }

    pub fn get_analysis_filepath<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let mut filepath = path.as_ref().join(&self.name);
        filepath.set_extension("ndjson");
        filepath
    }
}

impl RecordingStore {
    // Returns whether a directory with a "manifest.toml" exists at the given
    // path (though doesn't check if that manifest is valid)
    pub async fn exists<P>(path: P) -> Result<bool, RecordingStoreError>
    where
        P: AsRef<Path>,
    {
        let manifest_path = path.as_ref().join("manifest.toml");
        let dir_exists = try_exists(path)
            .await
            .map_err(RecordingStoreError::OpenDirError)?;
        let manifest_exists = try_exists(manifest_path)
            .await
            .map_err(RecordingStoreError::ReadManifestError)?;
        Ok(dir_exists && manifest_exists)
    }

    // Loads an existing RecordingStore at the given path. Errors if no store exists,
    // or if it's malformed.
    pub async fn load<P>(path: P) -> Result<Self, RecordingStoreError>
    where
        P: AsRef<Path>,
    {
        let path: PathBuf = path.as_ref().to_path_buf();
        let manifest = RecordingStore::read_manifest(&path).await?;
        Ok(RecordingStore {
            path,
            manifest,
            current_entry: None,
        })
    }

    // Creates a new RecordingStore at the given path. This involves creating a dir
    // and writing an empty manifest.
    pub async fn create<P>(path: P) -> Result<Self, RecordingStoreError>
    where
        P: AsRef<Path>,
    {
        let manifest_path = path.as_ref().join("manifest.toml");
        fs::create_dir_all(&path)
            .await
            .map_err(RecordingStoreError::OpenDirError)?;
        let mut manifest_file = File::create(&manifest_path)
            .await
            .map_err(RecordingStoreError::WriteManifestError)?;
        let empty_manifest = Manifest {
            entries: Vec::new(),
        };
        let empty_manifest_contents =
            toml::to_string_pretty(&empty_manifest).expect("failed to serialize manifest");
        manifest_file
            .write_all(empty_manifest_contents.as_bytes())
            .await
            .map_err(RecordingStoreError::WriteManifestError)?;
        RecordingStore::load(path).await
    }

    async fn read_manifest<P>(path: P) -> Result<Manifest, RecordingStoreError>
    where
        P: AsRef<Path>,
    {
        let manifest_path = path.as_ref().join("manifest.toml");
        let file_contents = fs::read_to_string(&manifest_path)
            .await
            .map_err(RecordingStoreError::ReadManifestError)?;
        toml::from_str(&file_contents).map_err(RecordingStoreError::ParseManifestError)
    }

    // Closes the current entry (if needed), creates a new entry based on the
    // current time, and updates the manifest. Returns a tuple of the entry's
    // newly created QMDL file and analysis file.
    pub async fn new_entry(&mut self) -> Result<(File, File), RecordingStoreError> {
        // if we've already got an entry open, close it
        if self.current_entry.is_some() {
            self.close_current_entry().await?;
        }
        let new_entry = ManifestEntry::new();
        let qmdl_filepath = new_entry.get_qmdl_filepath(&self.path);
        let qmdl_file = File::create(&qmdl_filepath)
            .await
            .map_err(RecordingStoreError::CreateFileError)?;
        let analysis_filepath = new_entry.get_analysis_filepath(&self.path);
        let analysis_file = File::create(&analysis_filepath)
            .await
            .map_err(RecordingStoreError::CreateFileError)?;
        self.manifest.entries.push(new_entry);
        self.current_entry = Some(self.manifest.entries.len() - 1);
        self.write_manifest().await?;
        Ok((qmdl_file, analysis_file))
    }

    // Returns the corresponding QMDL file for a given entry
    pub async fn open_entry_qmdl(
        &self,
        entry_index: usize,
    ) -> Result<File, RecordingStoreError> {
        let entry = &self.manifest.entries[entry_index];
        File::open(entry.get_qmdl_filepath(&self.path))
            .await
            .map_err(RecordingStoreError::ReadFileError)
    }

    // Returns the corresponding QMDL file for a given entry
    pub async fn open_entry_analysis(
        &self,
        entry_index: usize,
    ) -> Result<File, RecordingStoreError> {
        let entry = &self.manifest.entries[entry_index];
        File::open(entry.get_analysis_filepath(&self.path))
            .await
            .map_err(RecordingStoreError::ReadFileError)
    }

    pub async fn clear_and_open_entry_analysis(
        &mut self,
        entry_index: usize,
    ) -> Result<File, RecordingStoreError> {
        let entry = &self.manifest.entries[entry_index];
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(entry.get_analysis_filepath(&self.path))
            .await
            .map_err(RecordingStoreError::ReadFileError)?;
        self.update_entry_analysis_size(entry_index, 0)
            .await?;
        Ok(file)
    }

    // Unsets the current entry
    pub async fn close_current_entry(&mut self) -> Result<(), RecordingStoreError> {
        match self.current_entry {
            Some(_) => {
                self.current_entry = None;
                Ok(())
            }
            None => Err(RecordingStoreError::NoCurrentEntry),
        }
    }

    // Sets the given entry's size and updates the last_message_time to now, updating the manifest
    pub async fn update_entry_qmdl_size(
        &mut self,
        entry_index: usize,
        size_bytes: usize,
    ) -> Result<(), RecordingStoreError> {
        self.manifest.entries[entry_index].qmdl_size_bytes = size_bytes;
        self.manifest.entries[entry_index].last_message_time = Some(Local::now());
        self.write_manifest().await
    }

    // Sets the given entry's analysis file size
    pub async fn update_entry_analysis_size(
        &mut self,
        entry_index: usize,
        size_bytes: usize,
    ) -> Result<(), RecordingStoreError> {
        self.manifest.entries[entry_index].analysis_size_bytes = size_bytes;
        self.write_manifest().await
    }

    async fn write_manifest(&mut self) -> Result<(), RecordingStoreError> {
        let mut manifest_file = File::options()
            .write(true)
            .open(self.path.join("manifest.toml"))
            .await
            .map_err(RecordingStoreError::WriteManifestError)?;
        let manifest_contents =
            toml::to_string_pretty(&self.manifest).expect("failed to serialize manifest");
        manifest_file
            .write_all(manifest_contents.as_bytes())
            .await
            .map_err(RecordingStoreError::WriteManifestError)?;
        Ok(())
    }

    // Finds an entry by filename
    pub fn entry_for_name(&self, name: &str) -> Option<(usize, &ManifestEntry)> {
        let entry_index = self.manifest
            .entries
            .iter()
            .position(|entry| entry.name == name)?;
        Some((entry_index, &self.manifest.entries[entry_index]))
    }

    pub fn get_current_entry(&self) -> Option<(usize, &ManifestEntry)> {
        let entry_index = self.current_entry?;
        Some((entry_index, &self.manifest.entries[entry_index]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{Builder, TempDir};

    fn make_temp_dir() -> TempDir {
        Builder::new().prefix("qmdl_store_test").tempdir().unwrap()
    }

    #[tokio::test]
    async fn test_load_from_empty_dir() {
        let dir = make_temp_dir();
        assert!(!RecordingStore::exists(dir.path()).await.unwrap());
        let _created_store = RecordingStore::create(dir.path()).await.unwrap();
        assert!(RecordingStore::exists(dir.path()).await.unwrap());
        let loaded_store = RecordingStore::load(dir.path()).await.unwrap();
        assert_eq!(loaded_store.manifest.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_creating_updating_and_closing_entries() {
        let dir = make_temp_dir();
        let mut store = RecordingStore::create(dir.path()).await.unwrap();
        let _ = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        assert_eq!(
            RecordingStore::read_manifest(dir.path()).await.unwrap(),
            store.manifest
        );
        assert!(store.manifest.entries[entry_index]
            .last_message_time
            .is_none());

        store
            .update_entry_qmdl_size(entry_index, 1000)
            .await
            .unwrap();
        let (entry_index, entry) = store
            .entry_for_name(&store.manifest.entries[entry_index].name)
            .unwrap();
        assert!(entry.last_message_time.is_some());
        assert_eq!(store.manifest.entries[entry_index].qmdl_size_bytes, 1000);
        assert_eq!(
            RecordingStore::read_manifest(dir.path()).await.unwrap(),
            store.manifest
        );

        store.close_current_entry().await.unwrap();
        assert!(matches!(
            store.close_current_entry().await,
            Err(RecordingStoreError::NoCurrentEntry)
        ));
    }

    #[tokio::test]
    async fn test_create_on_existing_store() {
        let dir = make_temp_dir();
        let mut store = RecordingStore::create(dir.path()).await.unwrap();
        let _ = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        store
            .update_entry_qmdl_size(entry_index, 1000)
            .await
            .unwrap();
        let store = RecordingStore::create(dir.path()).await.unwrap();
        assert_eq!(store.manifest.entries.len(), 0);
    }

    #[tokio::test]
    async fn test_repeated_new_entries() {
        let dir = make_temp_dir();
        let mut store = RecordingStore::create(dir.path()).await.unwrap();
        let _ = store.new_entry().await.unwrap();
        let entry_index = store.current_entry.unwrap();
        let _ = store.new_entry().await.unwrap();
        let new_entry_index = store.current_entry.unwrap();
        assert_ne!(entry_index, new_entry_index);
        assert_eq!(store.manifest.entries.len(), 2);
    }
}
