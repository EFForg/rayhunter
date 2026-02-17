use std::collections::BTreeMap;

use crate::graph::DepGraph;

impl DepGraph {
    pub fn to_mermaid(&self, device_filter: Option<&str>, color_by: Option<&str>) -> String {
        let mut lines = vec!["graph TD".to_string()];

        if color_by == Some("maintainer") {
            lines.push("    classDef core fill:#2d9,stroke:#333,color:#000".to_string());
            lines.push("    classDef unmaintained fill:#f66,stroke:#333,color:#fff".to_string());
            lines.push("    classDef community fill:#fc3,stroke:#333,color:#000".to_string());
            lines.push("    classDef unknown fill:#ccc,stroke:#333,color:#000".to_string());
            lines.push(String::new());
            lines.push("    subgraph legend [\" \"]".to_string());
            lines.push("        L1[\"core maintained\"]:::core".to_string());
            lines.push("        L2[\"unmaintained\"]:::unmaintained".to_string());
            lines.push("        L3[\"community maintained\"]:::community".to_string());
            lines.push("    end".to_string());
        }

        let relevant = match device_filter {
            Some(device) => self.features_for_device(device),
            None => self.features.keys().cloned().collect(),
        };

        let mut categories: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for name in &relevant {
            if let Some(feat) = self.features.get(name) {
                categories
                    .entry(feat.category.as_str())
                    .or_default()
                    .push(name.as_str());
            }
        }

        for (cat, names) in &categories {
            lines.push(format!("    subgraph {cat}"));
            for &name in names {
                let label = name.replace('_', " ");
                let suffix = if color_by == Some("maintainer") {
                    format!(":::{}", self.maintainer_class(name))
                } else {
                    String::new()
                };
                lines.push(format!("        {name}[\"{label}\"]{suffix}"));
            }
            lines.push("    end".to_string());
        }

        for name in &relevant {
            if let Some(feat) = self.features.get(name) {
                let mut deps: Vec<&str> = feat.depends_on.iter().map(|s| s.as_str()).collect();
                deps.sort();
                for dep in deps {
                    if relevant.contains(dep) {
                        lines.push(format!("    {name} --> {dep}"));
                    }
                }
                let mut rdeps: Vec<&str> = feat.runtime_deps.iter().map(|s| s.as_str()).collect();
                rdeps.sort();
                for dep in rdeps {
                    if relevant.contains(dep) {
                        lines.push(format!("    {name} -.-> {dep}"));
                    }
                }
            }
        }

        lines.join("\n")
    }
}
