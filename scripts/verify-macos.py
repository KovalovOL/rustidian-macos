#!/usr/bin/env python3
"""Launch the packaged app with private profiles and verify its native chrome and themes."""
import argparse
import json
import os
import subprocess
import tempfile
import time
from pathlib import Path


def verify(app, output):
    app = app.resolve(strict=True)
    output = output.resolve()
    binary = app / "Contents/MacOS/rustidian"
    if not binary.is_file():
        raise SystemExit("Expected the packaged app's native executable")
    with tempfile.TemporaryDirectory(prefix="rd-qa-") as temporary:
        scratch = Path(temporary)
        for appearance, expected_color in (("light", [255, 255, 255, 255]), ("dark", [30, 30, 30, 255])):
            profile = scratch / appearance
            state = profile / "state/rustidian"
            state.mkdir(parents=True)
            (state / "appearance").write_text(json.dumps(appearance))
            (state / "palette").write_text(json.dumps("obsidian"))
            result = output / appearance
            result.mkdir(parents=True, exist_ok=True)
            document = profile / "files/From Finder.md"
            document.parent.mkdir()
            document.write_text("# Opened through Finder\n\nA native macOS Markdown notebook.\n\nУкраїнська мова: ї є ґ і ✅\n\n- [x] Native window controls\n- [x] Native menus\n- [x] Finder document delivery\n")
            env = dict(os.environ, XDG_DATA_HOME=str(profile / "data"), XDG_STATE_HOME=str(profile / "state"),
                XDG_CONFIG_HOME=str(profile / "config"), XDG_CACHE_HOME=str(profile / "cache"),
                RUSTIDIAN_MACOS_QA_DIR=str(result), RUSTIDIAN_MACOS_QA_EXIT="1",
                RUSTIDIAN_MACOS_QA_EXPECT_TITLE="From Finder")
            env.pop("RUSTIDIAN_DEFAULT_VAULT", None)
            env.pop("RUST_NOTES_DEFAULT_VAULT", None)
            with (result / "app.log").open("w") as log:
                process = subprocess.Popen([str(binary)], env=env, stdout=log, stderr=subprocess.STDOUT)
                try:
                    deadline = time.monotonic() + 30
                    while "first_ui_frame_prepared_us=" not in (result / "app.log").read_text():
                        if process.poll() is not None or time.monotonic() > deadline:
                            raise RuntimeError("The application did not finish its first frame")
                        time.sleep(0.05)
                    subprocess.run(["/usr/bin/open", "-a", str(app), str(document)], check=True, timeout=20)
                    assert process.wait(timeout=60) == 0
                finally:
                    if process.poll() is None:
                        process.terminate()
                        process.wait(timeout=10)
            report = json.loads((result / "native.json").read_text())
            assert report["native_chrome"] and report["menus_complete"], report
            assert report["traffic_lights_visible"] == [True, True, True], report
            assert report["dark"] == (appearance == "dark"), report
            assert report["document_background"] == expected_color, report
            assert "From Finder" in report["window_title"], report
            assert (result / "content.png").stat().st_size > 1000
            assert (profile / "data/rustidian/demo-vault").is_dir(), "First run must create a writable demo copy"
            print(f"PASS {appearance}: native traffic lights, menus, Finder/Open With, palette, demo setup, and rendered screenshot")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--app", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    verify(args.app, args.output)
