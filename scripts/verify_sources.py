"""Check preserved Rust implementations against an explicit local baseline."""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--baseline-dir", required=True, type=Path)
args = parser.parse_args()
records = {}
for crate in ("spindalis", "spindalis_core", "spindalis_macros"):
    sources = sorted((args.baseline_dir / crate / "src").rglob("*.rs"))
    assert sources, f"Missing baseline source: {crate}"
    for baseline in sources:
        relative = baseline.relative_to(args.baseline_dir)
        current = ROOT / relative
        assert current.read_bytes() == baseline.read_bytes(), str(relative)
        records[str(relative)] = hashlib.sha256(current.read_bytes()).hexdigest()
report = {"unchanged_rust_implementation_files": len(records), "sha256": records}
print(json.dumps(report, indent=2))
