# Third-party notices

Local Grind's own code and content are dual-licensed under MIT OR Apache-2.0 (see
`LICENSE-MIT` and `LICENSE-APACHE`). The distributed app also contains or is built from the
following third-party works, each under its own license.

| Component | Where | License |
|---|---|---|
| CPython 3.13 (python-build-standalone) | Bundled in the app (`Resources/python`) | Python Software Foundation License; full text ships at `Resources/python/lib/python3.13/LICENSE.txt`, which also covers the libraries CPython bundles |
| Inconsolata font | Bundled in the app's web assets via `@fontsource/inconsolata` | SIL Open Font License 1.1 (`app/node_modules/@fontsource/inconsolata/LICENSE`) |
| Tauri, CodeMirror, Svelte, rusqlite/SQLite, syntect and other Rust/npm dependencies | Compiled into the app | Their respective permissive licenses (MIT, Apache-2.0, BSD, Zlib, public domain), as declared in each package's metadata (`Cargo.lock`, `app/package-lock.json`) |
| Company tag data | `content/companies.json` | Facts derived from [liquidslr/leetcode-company-wise-problems](https://github.com/liquidslr/leetcode-company-wise-problems) (no license declared upstream); attribution kept here |

Problem statements, editorials, hints, test generators and reference solutions in `content/` are
original works of the Local Grind contributors describing well-known interview problems. LeetCode
is a trademark of its owner; Local Grind is not affiliated with LeetCode.
