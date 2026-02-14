#!/usr/bin/env python3
"""Rayhunter feature and device dependency graph tool.

Queries a deps.toml manifest to answer impact analysis questions
and generate Mermaid dependency diagrams.

Usage:
    python3 tools/deps.py validate
    python3 tools/deps.py impact <feature>
    python3 tools/deps.py deps <feature>
    python3 tools/deps.py device <device>
    python3 tools/deps.py graph [--device NAME] [--color-by maintainer]
"""

import sys
import argparse
from collections import defaultdict, deque
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    try:
        import tomli as tomllib
    except ModuleNotFoundError:
        sys.exit("Python 3.11+ required (or: pip install tomli)")


class Feature:
    __slots__ = (
        "name", "category", "description", "depends_on", "runtime_deps",
        "devices", "cargo_feature", "config_key", "source_files",
    )

    def __init__(self, name, data):
        self.name = name
        self.category = data["category"]
        self.description = data["description"]
        self.depends_on = data.get("depends_on", [])
        self.runtime_deps = data.get("runtime_deps", [])
        self.devices = data["devices"]
        self.cargo_feature = data.get("cargo_feature")
        self.config_key = data.get("config_key")
        self.source_files = data.get("source_files", [])


class Maintenance:
    __slots__ = ("maintainer", "author", "pr", "date")

    def __init__(self, data):
        self.maintainer = data.get("maintainer", "unknown")
        intro = data.get("introduced_by", {})
        self.author = intro.get("author", "unknown")
        self.pr = intro.get("pr")
        self.date = intro.get("date")


class DepGraph:
    def __init__(self, features, devices, maintenance=None):
        self.features = features
        self.devices = devices
        self.maintenance = maintenance or {}

        self.forward = defaultdict(set)
        self.reverse = defaultdict(set)
        self.runtime_forward = defaultdict(set)
        self.runtime_reverse = defaultdict(set)

        for name, feat in features.items():
            for dep in feat.depends_on:
                self.forward[name].add(dep)
                self.reverse[dep].add(name)
            for dep in feat.runtime_deps:
                self.runtime_forward[name].add(dep)
                self.runtime_reverse[dep].add(name)

    @classmethod
    def load(cls, deps_path, maint_path=None):
        with open(deps_path, "rb") as f:
            data = tomllib.load(f)

        devices = {}
        for name, meta in data.get("device", {}).items():
            devices[name] = meta

        features = {}
        for name, fdata in data.get("feature", {}).items():
            features[name] = Feature(name, fdata)

        maintenance = {}
        if maint_path and maint_path.exists():
            with open(maint_path, "rb") as f:
                mdata = tomllib.load(f)
            for name, minfo in mdata.get("feature", {}).items():
                maintenance[name] = Maintenance(minfo)

        return cls(features, devices, maintenance)

    def _traverse(self, name, adjacency):
        visited = set()
        queue = deque(adjacency.get(name, []))
        while queue:
            current = queue.popleft()
            if current in visited:
                continue
            visited.add(current)
            queue.extend(adjacency.get(current, []))
        return visited

    def transitive_deps(self, name, include_runtime=False):
        result = self._traverse(name, self.forward)
        if include_runtime:
            runtime = self._traverse(name, self.runtime_forward)
            result |= runtime
            for r in list(runtime):
                result |= self._traverse(r, self.forward)
        return result

    def transitive_dependents(self, name, include_runtime=False):
        result = self._traverse(name, self.reverse)
        if include_runtime:
            runtime = self._traverse(name, self.runtime_reverse)
            result |= runtime
            for r in list(runtime):
                result |= self._traverse(r, self.reverse)
        return result

    def features_for_device(self, device_name):
        return {
            name for name, feat in self.features.items()
            if device_name in feat.devices
        }

    def affected_devices(self, name):
        affected = self.transitive_dependents(name, include_runtime=True) | {name}
        devices = set()
        for fname in affected:
            if fname in self.features:
                devices.update(self.features[fname].devices)
        return devices

    def validate(self):
        errors = []

        for name, feat in self.features.items():
            for d in feat.devices:
                if d not in self.devices:
                    errors.append(
                        f"feature '{name}' references unknown device '{d}'"
                    )
            for dep in feat.depends_on:
                if dep not in self.features:
                    errors.append(
                        f"feature '{name}' depends_on unknown feature '{dep}'"
                    )
            for dep in feat.runtime_deps:
                if dep not in self.features:
                    errors.append(
                        f"feature '{name}' runtime_deps unknown feature '{dep}'"
                    )

        in_degree = {name: 0 for name in self.features}
        edges = defaultdict(set)
        for name, feat in self.features.items():
            for dep in feat.depends_on:
                if dep in self.features:
                    edges[dep].add(name)
                    in_degree[name] += 1

        queue = deque(n for n, d in in_degree.items() if d == 0)
        sorted_nodes = []
        while queue:
            node = queue.popleft()
            sorted_nodes.append(node)
            for dependent in edges.get(node, []):
                in_degree[dependent] -= 1
                if in_degree[dependent] == 0:
                    queue.append(dependent)

        if len(sorted_nodes) != len(self.features):
            cycle_members = set(self.features) - set(sorted_nodes)
            errors.append(
                f"dependency cycle involving: {', '.join(sorted(cycle_members))}"
            )

        if self.maintenance:
            for name in self.maintenance:
                if name not in self.features:
                    errors.append(
                        f"maintenance.toml references unknown feature '{name}'"
                    )
            for name in self.features:
                if name not in self.maintenance:
                    errors.append(
                        f"feature '{name}' has no maintenance.toml entry"
                    )

        return errors

    def _maintainer_class(self, name):
        if name not in self.maintenance:
            return "unknown"
        m = self.maintenance[name].maintainer
        if m == "core":
            return "core"
        if m == "unmaintained":
            return "unmaintained"
        if m.startswith("community:"):
            return "community"
        return "unknown"

    def to_mermaid(self, device_filter=None, color_by=None):
        lines = ["graph TD"]

        if color_by == "maintainer":
            lines.append('    classDef core fill:#2d9,stroke:#333,color:#000')
            lines.append('    classDef unmaintained fill:#f66,stroke:#333,color:#fff')
            lines.append('    classDef community fill:#fc3,stroke:#333,color:#000')
            lines.append('    classDef unknown fill:#ccc,stroke:#333,color:#000')
            lines.append('')
            lines.append('    subgraph legend [" "]')
            lines.append('        L1["core maintained"]:::core')
            lines.append('        L2["unmaintained"]:::unmaintained')
            lines.append('        L3["community maintained"]:::community')
            lines.append('    end')

        if device_filter:
            relevant = self.features_for_device(device_filter)
        else:
            relevant = set(self.features.keys())

        categories = defaultdict(list)
        for name in sorted(relevant):
            categories[self.features[name].category].append(name)

        for cat in sorted(categories.keys()):
            lines.append(f"    subgraph {cat}")
            for name in categories[cat]:
                label = name.replace("_", " ")
                suffix = ""
                if color_by == "maintainer":
                    suffix = f":::{self._maintainer_class(name)}"
                lines.append(f'        {name}["{label}"]{suffix}')
            lines.append("    end")

        for name in sorted(relevant):
            feat = self.features[name]
            for dep in sorted(feat.depends_on):
                if dep in relevant:
                    lines.append(f"    {name} --> {dep}")
            for dep in sorted(feat.runtime_deps):
                if dep in relevant:
                    lines.append(f"    {name} -.-> {dep}")

        return "\n".join(lines)


def find_manifest(override=None):
    if override:
        return Path(override)
    script_dir = Path(__file__).resolve().parent
    for candidate in [script_dir.parent / "deps.toml", script_dir / "deps.toml"]:
        if candidate.exists():
            return candidate
    sys.exit("Cannot find deps.toml")


def find_maintenance(deps_path):
    return deps_path.parent / "maintenance.toml"


def cmd_impact(graph, args):
    name = args.name
    if name not in graph.features:
        sys.exit(f"Unknown feature: {name}")

    dependents = graph.transitive_dependents(name, include_runtime=True)
    devices = graph.affected_devices(name)
    feat = graph.features[name]

    print(f"Impact of removing '{name}':")
    print(f"  {feat.description}")
    print()

    if dependents:
        print("  Features that would break:")
        for d in sorted(dependents):
            print(f"    - {d}: {graph.features[d].description}")
    else:
        print("  No other features depend on this.")

    print()
    print(f"  Affected devices: {', '.join(sorted(devices))}")

    if name in graph.maintenance:
        m = graph.maintenance[name]
        print()
        print(f"  Maintainer: {m.maintainer}")
        print(f"  Introduced by: {m.author}", end="")
        if m.pr:
            print(f" (PR #{m.pr})", end="")
        if m.date:
            print(f" on {m.date}", end="")
        print()


def cmd_deps(graph, args):
    name = args.name
    if name not in graph.features:
        sys.exit(f"Unknown feature: {name}")

    deps = graph.transitive_deps(name)
    runtime = graph.transitive_deps(name, include_runtime=True) - deps
    feat = graph.features[name]

    print(f"Dependencies of '{name}':")
    print(f"  {feat.description}")
    print()
    print(f"  Direct: {', '.join(sorted(feat.depends_on)) or '(none)'}")
    print(f"  Runtime: {', '.join(sorted(feat.runtime_deps)) or '(none)'}")
    print(f"  All transitive: {', '.join(sorted(deps | runtime)) or '(none)'}")


def cmd_device(graph, args):
    name = args.name
    if name not in graph.devices:
        sys.exit(f"Unknown device: {name}")

    features = graph.features_for_device(name)
    categories = defaultdict(list)
    for fname in sorted(features):
        categories[graph.features[fname].category].append(fname)

    desc = graph.devices[name].get("description", "")
    print(f"Features for device '{name}' ({desc}):")
    print()
    for cat in sorted(categories.keys()):
        print(f"  [{cat}]")
        for fname in categories[cat]:
            maint_tag = ""
            if fname in graph.maintenance:
                m = graph.maintenance[fname]
                if m.maintainer == "unmaintained":
                    maint_tag = " [UNMAINTAINED]"
                elif m.maintainer.startswith("community:"):
                    maint_tag = f" [{m.maintainer}]"
            print(f"    - {fname}{maint_tag}")
        print()


def cmd_graph(graph, args):
    print(graph.to_mermaid(device_filter=args.device, color_by=args.color_by))


def cmd_validate(graph, args):
    errors = graph.validate()
    if errors:
        print(f"Found {len(errors)} error(s):")
        print()
        for e in errors:
            print(f"  - {e}")
        sys.exit(1)
    else:
        maint_note = ""
        if graph.maintenance:
            maint_note = f", {len(graph.maintenance)} maintenance entries"
        print(f"Valid. {len(graph.features)} features, {len(graph.devices)} devices{maint_note}.")


def main():
    parser = argparse.ArgumentParser(description="Rayhunter dependency graph tool")
    parser.add_argument("--manifest", default=None, help="Path to deps.toml")
    sub = parser.add_subparsers(dest="command", required=True)

    p = sub.add_parser("impact", help="Show what breaks if a feature is removed")
    p.add_argument("name")

    p = sub.add_parser("deps", help="Show transitive dependencies of a feature")
    p.add_argument("name")

    p = sub.add_parser("device", help="Show all features for a device")
    p.add_argument("name")

    p = sub.add_parser("graph", help="Output Mermaid diagram")
    p.add_argument("--device", default=None, help="Filter to one device")
    p.add_argument("--color-by", default=None, choices=["maintainer"],
                   help="Color nodes by maintenance status")

    sub.add_parser("validate", help="Check manifest for errors")

    args = parser.parse_args()
    manifest = find_manifest(args.manifest)
    maint = find_maintenance(manifest)
    graph = DepGraph.load(manifest, maint)

    commands = {
        "impact": cmd_impact,
        "deps": cmd_deps,
        "device": cmd_device,
        "graph": cmd_graph,
        "validate": cmd_validate,
    }
    commands[args.command](graph, args)


if __name__ == "__main__":
    main()
