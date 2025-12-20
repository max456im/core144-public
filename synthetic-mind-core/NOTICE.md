Synthetic Mind Core (Private)
Copyright 2025 Maxim Zapevalov

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

──────────────────────────────────────────────

The following components of this project are licensed exclusively under
the Apache License, Version 2.0:

- synthetic-mind-core/ (the private core library)
  ├── src/ontology/
  ├── src/phase_stack/
  ├── src/ontic_groups/
  ├── src/reflexive_core/
  ├── src/internal_time/
  ├── src/multimodal_invariants/
  ├── src/synthetic_morality/
  ├── src/memory/
  └── src/utils/ (only true utilities: bit operations, hashing, etc.)

- interfaces/ (ontologically neutral data structures for inter-module communication)

- runtime/ (IPC and plugin loader for GPL isolation)

- All products and subprojects listed below:
  ├── products/minimal-agent/
  ├── products/synthetic-transponder/
  └── products/shadow-subarchitecture/

- All documentation, examples, and build scripts not explicitly marked otherwise:
  ├── docs/core-concepts/
  ├── docs/architecture.md
  ├── examples/
  └── scripts/build_no_std.sh

These components contain no code derived from GPL- or MPL-licensed sources
and may be used, modified, and redistributed under the terms of the Apache License 2.0.

──────────────────────────────────────────────

Note: This project also includes separate components under MPL 2.0
(language-module/, meta-ontology-module/) and GPL v3 (gpl-modules/),
which are distributed as independent modules and are not part of the
Apache-licensed core. They are not included in the above list.