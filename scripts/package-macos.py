#!/usr/bin/env python3
"""Build a self-contained macOS .app and ZIP from an already verified binary."""
import argparse
import hashlib
import plistlib
import shutil
import struct
import subprocess
import tempfile
from pathlib import Path


def package(binary, output, identity="-"):
    root = Path(__file__).resolve().parents[1]
    binary = binary.resolve(strict=True)
    output = output.resolve()
    if output.exists():
        raise SystemExit(f"Refusing to overwrite {output}")
    if output.suffix != ".app":
        raise SystemExit("--output must end in .app")
    version = next(line.split('"')[1] for line in (root / "Cargo.toml").read_text().splitlines() if line.startswith("version = "))
    architectures = subprocess.check_output(["/usr/bin/lipo", "-archs", str(binary)], text=True).strip()
    if not set(architectures.split()) <= {"arm64", "x86_64"}:
        raise SystemExit("Expected an Apple Silicon, Intel, or universal macOS binary")
    macos = output / "Contents/MacOS"
    resources = output / "Contents/Resources"
    macos.mkdir(parents=True)
    resources.mkdir()
    shutil.copyfile(binary, macos / "rustidian")
    (macos / "rustidian").chmod(0o755)
    for name in ("LICENSE", "THIRD_PARTY_NOTICES.txt"):
        shutil.copyfile(root / name, resources / name)
    shutil.copytree(root / "demo-vault", resources / "demo-vault", copy_function=shutil.copyfile)
    licenses = resources / "licenses"
    for source in [*(root / "assets/fonts").glob("LICENSE*.txt"),
                   root / "assets/fonts/NOTICE.txt", root / "assets/brand/NOTICE.txt",
                   root / "vendor/LICENSE-MIT", root / "vendor/LICENSE-APACHE",
                   root / "vendor/NOTICE.txt"]:
        destination = licenses / source.relative_to(root)
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
    with tempfile.TemporaryDirectory(prefix="rustidian-icon-") as scratch:
        png = Path(scratch) / "icon.png"
        subprocess.run(["/usr/bin/sips", "-z", "1024", "1024", str(root / "assets/brand/themes/signature.png"), "--out", str(png)], check=True, capture_output=True)
        data = png.read_bytes()
        chunk = struct.pack(">4sI", b"ic10", len(data) + 8) + data
        (resources / "Rustidian.icns").write_bytes(struct.pack(">4sI", b"icns", len(chunk) + 8) + chunk)
    info = {
        "CFBundleName": "Rustidian", "CFBundleDisplayName": "Rustidian",
        "CFBundleIdentifier": "io.github.kovalovol.rustidian-macos",
        "CFBundleExecutable": "rustidian", "CFBundlePackageType": "APPL",
        "CFBundleVersion": version.split("-")[0], "CFBundleShortVersionString": version.split("-")[0],
        "CFBundleIconFile": "Rustidian.icns", "LSMinimumSystemVersion": "12.0",
        "LSApplicationCategoryType": "public.app-category.productivity",
        "NSHighResolutionCapable": True, "NSPrincipalClass": "NSApplication",
        "CFBundleDocumentTypes": [{"CFBundleTypeName": "Markdown and Plain Text", "CFBundleTypeRole": "Editor",
            "LSHandlerRank": "Alternate", "LSItemContentTypes": ["net.daringfireball.markdown", "public.plain-text"],
            "CFBundleTypeExtensions": ["md", "markdown", "txt"]}],
    }
    (output / "Contents/Info.plist").write_bytes(plistlib.dumps(info))
    command = ["/usr/bin/codesign", "--force", "--deep", "--sign", identity]
    command += ["--options", "runtime", "--timestamp"] if identity != "-" else ["--timestamp=none"]
    subprocess.run(command + [str(output)], check=True)
    subprocess.run(["/usr/bin/codesign", "--verify", "--deep", "--strict", str(output)], check=True)
    archive = output.with_suffix(".zip")
    subprocess.run(["/usr/bin/ditto", "-c", "-k", "--sequesterRsrc", "--keepParent", str(output), str(archive)], check=True)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    archive.with_suffix(".zip.sha256").write_text(f"{checksum}  {archive.name}\n")
    print(f"Packaged {architectures}: {archive}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--sign-identity", default="-", help="Developer ID identity, or '-' for a local ad-hoc build")
    args = parser.parse_args()
    package(args.binary, args.output, args.sign_identity)
