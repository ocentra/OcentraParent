<!-- agent-capsule -->

> Agent Capsule
> Doc: Repo Mindmap
> Kind: visual repository navigation map.
> Read when: You need to locate product truth, architecture, modules, features, plans, or proof records.
> Stop rule: Follow the relevant branch of the map; do not bulk-load unrelated docs.
> Proves: navigation only.
> Does not prove: implementation, validation, or product status.

<!-- /agent-capsule -->

# Repo Mindmap

This map connects the product front door to architecture, modules, features, expectations, plans, and proof records.

```mermaid
flowchart TB
  Root["README.md\nproduct front door"]
  Start["docs/START_HERE.md\nshort route"]
  Constitution["product-constitution.md\nclaims + status words"]
  Roadmap["product-roadmap.md\nmilestone order"]
  Checklist["product-capability-checklist.md\ncurrent status"]
  FeatureList["feature-list.md\nfeature catalog"]
  FeatureRoute["FEATURE_ROUTE_INDEX.md\nfeature -> docs/plans"]
  Expectations["feature-expectations.md\nexpectation index"]
  System["architecture/system-overview.md\nend-to-end flow"]
  Boundaries["DEPENDENCY_BOUNDARY_MATRIX.md\nallowed dependencies"]
  Events["EVENT_FLOW_MAP.md\ncommand/event/read-model flow"]
  Modules["MODULE_MAP.md\narea -> modules/plans"]
  PlanMap["MODULE_PLAN_MAP.md\nmodule -> plan route"]
  Coverage["MODULE_README_COVERAGE.md\napp/package/crate targets"]
  Apps["apps/README.md"]
  Packages["packages/README.md"]
  Crates["crates/README.md"]
  Plans["PLAN_INDEX.md\nplan routes"]
  Proof["checkpoints / output / test-results\nproof records"]

  Root --> Start
  Start --> Constitution
  Start --> Roadmap
  Start --> Checklist
  Start --> FeatureList
  Start --> System
  Start --> Modules

  FeatureList --> FeatureRoute --> Plans --> Proof
  FeatureList --> Expectations
  Checklist --> Proof

  System --> Boundaries
  System --> Events
  Modules --> PlanMap --> Plans
  Modules --> Coverage
  Modules --> Apps
  Modules --> Packages
  Modules --> Crates
  Apps --> PlanMap
  Packages --> PlanMap
  Crates --> PlanMap
```

## Route Rules

- Product claims route through constitution, roadmap, feature docs, and capability checklist.
- Engineering ownership routes through module map, module plan map, workspace READMEs, dependency matrix, and event-flow map.
- Execution routes through plan index and a single owning plan workpack.
- Proof routes through checklist rows, feature docs, plan state, and named artifacts.

## Do Not Treat As Completion

This map does not prove product readiness. It only shows where to read next.
