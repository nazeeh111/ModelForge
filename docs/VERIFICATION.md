# Verification

Verified locally on 2026-09-22 using Rust 1.98.1 on macOS ARM64.

- Original workspace: 339 tests passed before changes.
- Final workspace: 342 tests passed, including three new facade tests covering exact floating-point results, derivative structure, and both branded polynomial macros.
- At that September 22 snapshot, all 57 Rust implementation source files in the three compatibility crates were byte-identical to the baseline. [Historical source hashes](source-parity.json). The later solver changes below intentionally differ.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- The lockfile records the resolved dependency graph.

The ModelForge facade re-exports existing operations. Branded procedural macros retain the same parser and generated values while pointing generated type names at the public ModelForge namespace. This allows applications to use the macros without directly adding a core crate dependency. Existing crate APIs remain available.

## Solver convergence repairs, 2026-10-02

[PR #1](https://github.com/nazeeh111/ModelForge/pull/1), merged as `a2e4cef0310f76b6564eef544c071d755b4408ef`, corrects bisection stopping at an arbitrary midpoint initial guess and Newton rejection of an exact zero root. It also preserves the iteration-budget contract. Seven focused regression cases and the existing five-package build/test and Clippy jobs passed in GitHub Actions on the merged commit. The first six cases had failed against the original solvers; an additional zero-budget case caught a regression during review before the final repair.

This was hosted Rust execution, not a new local run. The two changed implementation files are `spindalis/src/solvers/bisection.rs` and `spindalis/src/solvers/nrm.rs`; all other retained implementation files remain unchanged. The historical source-parity record above predates these intentional fixes. Comparing against that original baseline will therefore report these two differences. These bounded checks do not establish universal numerical correctness.

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
