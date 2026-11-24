"""Safe renderer smoke test: only synthetic fixtures, no file operations."""
import argparse
import os
import subprocess
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("renderer", type=Path)
args = p.parse_args()
exe = args.renderer.resolve()
root = Path(__file__).resolve().parents[1] / "lf-integration-poc"

def preview(path, backend="ansi"):
    env = dict(os.environ, LFP_IMAGE_BACKEND=backend)
    return subprocess.check_output(
        [str(exe), str(root / path), "72", "45", "0", "0", "preview"],
        env=env, timeout=15,
    )

markdown = preview("fixtures-writer/01-markdown.md")
diagram = preview("fixtures-writer/02-math-mermaid.md")
assert b"README.md" in markdown and "━".encode() in markdown
assert all(s.encode() in diagram for s in ("α²", "β²", "Read", "Render", "Preview", "┌"))
assert b"\x1bP9;1;0q" in preview("fixtures-image/01 gradient.png", "sixel")
assert "▀".encode() in preview("fixtures-image/01 gradient.png")
print("PASS: Markdown/table, math/Mermaid, Sixel, ANSI (4/4)")
