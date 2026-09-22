# Verification

Verified locally on 2026-09-22 using Rust 1.98.1 on macOS ARM64.

- Original workspace: 339 tests passed before changes.
- Final workspace: 342 tests passed, including three new facade tests covering exact floating-point results, derivative structure, and both branded polynomial macros.
- All 57 Rust implementation source files in the three compatibility crates are byte-identical to the baseline. [Source hashes](source-parity.json).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- The lockfile records the resolved dependency graph.

The ModelForge facade re-exports existing operations. Branded procedural macros retain the same parser and generated values while pointing generated type names at the public ModelForge namespace. This allows applications to use the macros without directly adding a core crate dependency. Existing crate APIs remain available.

## Repeat checks

```bash
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

To compare retained implementation sources with a separate prior checkout:

```bash
python3 scripts/verify_sources.py --baseline-dir /path/to/previous-checkout
```

The baseline must contain the original three crate source directories. The comparison requires an explicit local baseline, not historical Git objects. Normal tests and builds work in a clean clone.

## Limits

Tests cover the existing library suite and the new namespace paths; they are not a proof for every possible numerical input or platform. Advanced polynomial support remains limited to the implemented operations. Scientific results and the behavior of external applications have not been independently validated. Only the reported macOS/Rust environment was executed locally.
