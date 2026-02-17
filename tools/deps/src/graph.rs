use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    #[allow(dead_code)]
    pub meta: Meta,
    pub device: BTreeMap<String, DeviceMeta>,
    pub feature: BTreeMap<String, FeatureData>,
}

#[derive(Deserialize)]
pub struct Meta {
    #[allow(dead_code)]
    pub version: u32,
}

#[derive(Deserialize)]
pub struct DeviceMeta {
    pub description: String,
    pub rust_variant: String,
}

#[derive(Deserialize)]
pub struct FeatureData {
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub runtime_deps: Vec<String>,
    pub devices: Vec<String>,
    #[allow(dead_code)]
    pub cargo_feature: Option<String>,
    #[allow(dead_code)]
    pub config_key: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub source_files: Vec<String>,
}

#[derive(Deserialize)]
pub struct MaintenanceManifest {
    pub feature: BTreeMap<String, MaintenanceEntry>,
}

#[derive(Deserialize)]
pub struct MaintenanceEntry {
    pub maintainer: Option<String>,
    pub introduced_by: Option<IntroducedBy>,
}

#[derive(Deserialize)]
pub struct IntroducedBy {
    pub author: Option<String>,
    pub pr: Option<u32>,
    pub date: Option<String>,
}

pub struct DepGraph {
    pub features: BTreeMap<String, FeatureData>,
    pub devices: BTreeMap<String, DeviceMeta>,
    pub maintenance: BTreeMap<String, MaintenanceEntry>,
    forward: HashMap<String, HashSet<String>>,
    reverse: HashMap<String, HashSet<String>>,
    runtime_forward: HashMap<String, HashSet<String>>,
    runtime_reverse: HashMap<String, HashSet<String>>,
}

impl DepGraph {
    pub fn load(deps_path: &Path, maint_path: &Path) -> Result<Self, String> {
        let deps_content =
            fs::read_to_string(deps_path).map_err(|e| format!("Cannot read deps.toml: {e}"))?;
        let manifest: Manifest =
            toml::from_str(&deps_content).map_err(|e| format!("Cannot parse deps.toml: {e}"))?;

        let maintenance = if maint_path.exists() {
            let maint_content = fs::read_to_string(maint_path)
                .map_err(|e| format!("Cannot read maintenance.toml: {e}"))?;
            let maint: MaintenanceManifest = toml::from_str(&maint_content)
                .map_err(|e| format!("Cannot parse maintenance.toml: {e}"))?;
            maint.feature
        } else {
            BTreeMap::new()
        };

        let mut forward: HashMap<String, HashSet<String>> = HashMap::new();
        let mut reverse: HashMap<String, HashSet<String>> = HashMap::new();
        let mut runtime_forward: HashMap<String, HashSet<String>> = HashMap::new();
        let mut runtime_reverse: HashMap<String, HashSet<String>> = HashMap::new();

        for (name, feat) in &manifest.feature {
            for dep in &feat.depends_on {
                forward.entry(name.clone()).or_default().insert(dep.clone());
                reverse.entry(dep.clone()).or_default().insert(name.clone());
            }
            for dep in &feat.runtime_deps {
                runtime_forward
                    .entry(name.clone())
                    .or_default()
                    .insert(dep.clone());
                runtime_reverse
                    .entry(dep.clone())
                    .or_default()
                    .insert(name.clone());
            }
        }

        Ok(DepGraph {
            features: manifest.feature,
            devices: manifest.device,
            maintenance,
            forward,
            reverse,
            runtime_forward,
            runtime_reverse,
        })
    }

    fn traverse(
        &self,
        name: &str,
        adjacency: &HashMap<String, HashSet<String>>,
    ) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<String> = adjacency
            .get(name)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        while let Some(current) = queue.pop_front() {
            if !visited.insert(current.clone()) {
                continue;
            }
            if let Some(neighbors) = adjacency.get(&current) {
                queue.extend(neighbors.iter().cloned());
            }
        }
        visited
    }

    pub fn transitive_deps(&self, name: &str, include_runtime: bool) -> HashSet<String> {
        let mut result = self.traverse(name, &self.forward);
        if include_runtime {
            let runtime = self.traverse(name, &self.runtime_forward);
            for r in &runtime {
                result.extend(self.traverse(r, &self.forward));
            }
            result.extend(runtime);
        }
        result
    }

    pub fn transitive_dependents(&self, name: &str, include_runtime: bool) -> HashSet<String> {
        let mut result = self.traverse(name, &self.reverse);
        if include_runtime {
            let runtime = self.traverse(name, &self.runtime_reverse);
            for r in &runtime {
                result.extend(self.traverse(r, &self.reverse));
            }
            result.extend(runtime);
        }
        result
    }

    pub fn features_for_device(&self, device: &str) -> BTreeSet<String> {
        self.features
            .iter()
            .filter(|(_, feat)| feat.devices.contains(&device.to_string()))
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn affected_devices(&self, name: &str) -> BTreeSet<String> {
        let mut affected = self.transitive_dependents(name, true);
        affected.insert(name.to_string());
        let mut devices = BTreeSet::new();
        for fname in &affected {
            if let Some(feat) = self.features.get(fname) {
                devices.extend(feat.devices.iter().cloned());
            }
        }
        devices
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for (name, feat) in &self.features {
            for d in &feat.devices {
                if !self.devices.contains_key(d) {
                    errors.push(format!("feature '{name}' references unknown device '{d}'"));
                }
            }
            for dep in &feat.depends_on {
                if !self.features.contains_key(dep) {
                    errors.push(format!(
                        "feature '{name}' depends_on unknown feature '{dep}'"
                    ));
                }
            }
            for dep in &feat.runtime_deps {
                if !self.features.contains_key(dep) {
                    errors.push(format!(
                        "feature '{name}' runtime_deps unknown feature '{dep}'"
                    ));
                }
            }
        }

        // Cycle detection via topological sort
        let mut in_degree: HashMap<&str, usize> =
            self.features.keys().map(|k| (k.as_str(), 0)).collect();
        let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
        for (name, feat) in &self.features {
            for dep in &feat.depends_on {
                if self.features.contains_key(dep) {
                    edges.entry(dep.as_str()).or_default().push(name.as_str());
                    *in_degree.entry(name.as_str()).or_default() += 1;
                }
            }
        }

        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(n, _)| *n)
            .collect();
        let mut sorted_count = 0;
        while let Some(node) = queue.pop_front() {
            sorted_count += 1;
            if let Some(dependents) = edges.get(node) {
                for &dep in dependents {
                    let deg = in_degree.get_mut(dep).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dep);
                    }
                }
            }
        }

        if sorted_count != self.features.len() {
            let cycle_members: BTreeSet<&str> = in_degree
                .iter()
                .filter(|(_, d)| **d > 0)
                .map(|(n, _)| *n)
                .collect();
            errors.push(format!(
                "dependency cycle involving: {}",
                cycle_members.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }

        for (name, meta) in &self.devices {
            if meta.rust_variant.is_empty() {
                errors.push(format!("device '{name}' is missing rust_variant"));
            }
        }

        if !self.maintenance.is_empty() {
            for name in self.maintenance.keys() {
                if !self.features.contains_key(name) {
                    errors.push(format!(
                        "maintenance.toml references unknown feature '{name}'"
                    ));
                }
            }
            for name in self.features.keys() {
                if !self.maintenance.contains_key(name) {
                    errors.push(format!("feature '{name}' has no maintenance.toml entry"));
                }
            }
        }

        errors
    }

    pub fn maintainer_class(&self, name: &str) -> &'static str {
        match self.maintenance.get(name) {
            None => "unknown",
            Some(entry) => match entry.maintainer.as_deref() {
                Some("core") => "core",
                Some("unmaintained") => "unmaintained",
                Some(m) if m.starts_with("community:") => "community",
                _ => "unknown",
            },
        }
    }
}
