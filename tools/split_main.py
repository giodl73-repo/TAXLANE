#!/usr/bin/env python3
"""Split tools/taxlane/src/main.rs into ROUTE-style domain modules."""
from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

SRC = Path(__file__).resolve().parent / "taxlane" / "src"
MAIN = SRC / "main.rs"


def brace_end(lines: list[str], start: int) -> int:
    bal = 0
    started = False
    for j in range(start, len(lines)):
        for ch in lines[j]:
            if ch == "{":
                bal += 1
                started = True
            elif ch == "}":
                bal -= 1
                if started and bal == 0:
                    return j
    raise SystemExit(f"unclosed at {start + 1}: {lines[start][:100]}")


def item_start(lines: list[str], i: int) -> int:
    s = i
    while s > 0:
        prev = lines[s - 1].lstrip()
        if (
            prev.startswith("///")
            or prev.startswith("//!")
            or prev.startswith("#[")
            or (prev.startswith("//") and "moved" not in prev and prev.strip() != "//")
        ):
            s -= 1
            continue
        break
    return s


def const_end(lines: list[str], i: int) -> int:
    # const may be `= value;` or `= [...];` / `= &[...];` with braces/brackets
    j = i
    bracket = 0
    brace = 0
    paren = 0
    seen_eq = False
    while j < len(lines):
        line = lines[j]
        if "=" in line:
            seen_eq = True
        for ch in line:
            if ch == "[":
                bracket += 1
            elif ch == "]":
                bracket -= 1
            elif ch == "{":
                brace += 1
            elif ch == "}":
                brace -= 1
            elif ch == "(":
                paren += 1
            elif ch == ")":
                paren -= 1
        if seen_eq and bracket == 0 and brace == 0 and paren == 0 and ";" in line:
            return j
        j += 1
    raise SystemExit(f"unterminated const at {i + 1}")


def make_pub_crate_item(chunk: str, kind: str, name: str) -> str:
    patterns = [
        (rf"^(pub\(crate\)\s+|pub\s+)?fn {re.escape(name)}\b", f"pub(crate) fn {name}"),
        (rf"^(pub\(crate\)\s+|pub\s+)?struct {re.escape(name)}\b", f"pub(crate) struct {name}"),
        (rf"^(pub\(crate\)\s+|pub\s+)?enum {re.escape(name)}\b", f"pub(crate) enum {name}"),
        (rf"^(pub\(crate\)\s+|pub\s+)?const {re.escape(name)}\b", f"pub(crate) const {name}"),
        (rf"^(pub\(crate\)\s+|pub\s+)?static {re.escape(name)}\b", f"pub(crate) static {name}"),
        (rf"^(pub\(crate\)\s+|pub\s+)?type {re.escape(name)}\b", f"pub(crate) type {name}"),
    ]
    out = chunk
    for pat, rep in patterns:
        out2 = re.sub(pat, rep, out, count=1, flags=re.M)
        if out2 != out:
            out = out2
            break

    if kind != "struct":
        return out

    new_lines: list[str] = []
    in_body = False
    bal = 0
    for line in out.splitlines(keepends=True):
        if re.search(rf"\bstruct {re.escape(name)}\b", line):
            in_body = "{" in line or in_body
            if "{" in line:
                in_body = True
        if in_body:
            bal += line.count("{") - line.count("}")
            stripped = line.lstrip()
            if (
                stripped
                and not stripped.startswith("//")
                and not stripped.startswith("#[")
                and not stripped.startswith("}")
            ):
                m = re.match(
                    r"^(\s+)((?:pub(?:\([^)]*\))?\s+)?)([A-Za-z_][\w]*\s*:)",
                    line,
                )
                if m:
                    indent, vis, rest = m.group(1), m.group(2), m.group(3)
                    if not vis.strip():
                        line = f"{indent}pub(crate) {rest}{line[m.end() :]}"
                    elif vis.strip() == "pub":
                        line = f"{indent}pub(crate) {rest}{line[m.end() :]}"
            if bal <= 0:
                in_body = False
        new_lines.append(line)
    return "".join(new_lines)


def domain_for_fn(name: str) -> str | None:
    if name == "main":
        return None
    if name.startswith("run_"):
        return "commands"
    if name.startswith("validate_"):
        key = name[len("validate_") :].split("_")[0]
        return f"validate_{key}"
    if name.startswith("build_"):
        return "build"
    if name.startswith("check_"):
        return "check"
    if name.startswith(
        (
            "parse_",
            "read_",
            "write_",
            "assert_",
            "repo_",
            "chart_",
            "load_",
            "hash_",
            "open_",
            "path_",
        )
    ):
        return "util"
    return "misc"


def path_for(d: str) -> Path:
    if d == "commands":
        return SRC / "commands.rs"
    if d == "types":
        return SRC / "types.rs"
    if d == "artifacts":
        return SRC / "artifacts.rs"
    if d.startswith("validate_"):
        (SRC / "support" / "validate").mkdir(parents=True, exist_ok=True)
        return SRC / "support" / "validate" / f"{d[len('validate_') :]}.rs"
    (SRC / "support").mkdir(parents=True, exist_ok=True)
    return SRC / "support" / f"{d}.rs"


def main() -> None:
    text = MAIN.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines(keepends=True)

    # capture taxlane_core import block
    core_import = ""
    for idx, l in enumerate(lines[:250]):
        if "use taxlane_core::{" in l:
            end = idx
            while end < len(lines) and "};" not in lines[end]:
                end += 1
            core_import = "".join(lines[idx : end + 1])
            break

    items: list[dict] = []
    i = 0
    while i < len(lines):
        raw = lines[i].rstrip("\r\n")
        m = re.match(
            r"^(?:pub(?:\(crate\))?\s+)?(fn|struct|enum|const|static|type)\s+(\w+)",
            raw,
        )
        if m:
            kind, name = m.group(1), m.group(2)
            s = item_start(lines, i)
            if kind == "fn":
                e = brace_end(lines, i)
            elif kind in ("struct", "enum"):
                if "{" not in raw and ";" in raw:
                    e = i
                else:
                    e = brace_end(lines, i)
            elif kind in ("const", "static", "type"):
                e = const_end(lines, i)
            else:
                e = i
            items.append({"kind": kind, "name": name, "s": s, "e": e, "i": i})
            i = e + 1
            continue
        if raw.startswith("impl ") or raw.startswith("impl<"):
            e = brace_end(lines, i)
            nm = re.search(r"\bfor\s+(\w+)", raw) or re.search(
                r"impl(?:\s*<[^>]+>)?\s+(\w+)", raw
            )
            name = nm.group(1) if nm else "impl"
            s = item_start(lines, i)
            items.append({"kind": "impl", "name": name, "s": s, "e": e, "i": i})
            i = e + 1
            continue
        i += 1

    print(f"items {len(items)}")
    buckets: dict[str, list[dict]] = defaultdict(list)
    keep_main = None
    for it in items:
        if it["kind"] == "fn" and it["name"] == "main":
            keep_main = it
            continue
        if it["kind"] == "fn":
            d = domain_for_fn(it["name"]) or "misc"
        elif it["kind"] in ("struct", "enum", "type", "impl"):
            d = "types"
        elif it["kind"] in ("const", "static"):
            d = "artifacts"
        else:
            d = "misc"
        buckets[d].append(it)

    tiny = [d for d, v in list(buckets.items()) if d.startswith("validate_") and len(v) < 3]
    for d in tiny:
        buckets["validate_misc"].extend(buckets.pop(d))

    for d in buckets:
        buckets[d].sort(key=lambda x: x["s"])

    header = f"""//! Auto-split from main.rs (ROUTE-style domain layout).
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::*;
use std::env;
use std::fs::{{self, File}};
use std::io::Read;
use std::path::{{Path, PathBuf}};
use std::process::ExitCode;
use roxmltree::Document;
use sha2::{{Digest, Sha256}};
use std::collections::{{BTreeMap, BTreeSet}};
{core_import}
use zip::ZipArchive;

"""

    written = 0
    for d, its in sorted(buckets.items(), key=lambda kv: kv[0]):
        chunks: list[str] = []
        for it in its:
            chunk = "".join(lines[it["s"] : it["e"] + 1])
            if it["kind"] != "impl":
                chunk = make_pub_crate_item(chunk, it["kind"], it["name"])
            chunks.append(chunk.rstrip() + "\n\n")
        p = path_for(d)
        p.parent.mkdir(parents=True, exist_ok=True)
        body = header + "".join(chunks)
        p.write_text(body, encoding="utf-8")
        written += 1
        print(f"wrote {p.relative_to(SRC)} items={len(its)} lines={body.count(chr(10)) + 1}")

    # support/validate/mod.rs
    vdir = SRC / "support" / "validate"
    if vdir.exists():
        mods = sorted(p.stem for p in vdir.glob("*.rs") if p.name != "mod.rs")
        (vdir / "mod.rs").write_text(
            "#![allow(unused_imports)]\n"
            + "".join(f"pub(crate) mod {m};\n" for m in mods)
            + "".join(f"pub(crate) use {m}::*;\n" for m in mods),
            encoding="utf-8",
        )

    # support/mod.rs
    support = SRC / "support"
    smods: list[str] = []
    for p in sorted(support.iterdir()):
        if p.name == "mod.rs":
            continue
        if p.is_dir():
            smods.append(p.name)
        elif p.suffix == ".rs":
            smods.append(p.stem)
    (support / "mod.rs").write_text(
        "#![allow(unused_imports)]\n"
        + "".join(f"pub(crate) mod {m};\n" for m in smods)
        + "".join(f"pub(crate) use {m}::*;\n" for m in smods),
        encoding="utf-8",
    )

    assert keep_main is not None
    main_body = "".join(lines[keep_main["s"] : keep_main["e"] + 1])
    new_main = f"""use std::process::ExitCode;

mod artifacts;
mod commands;
mod support;
mod types;

pub(crate) use artifacts::*;
pub(crate) use commands::*;
pub(crate) use support::*;
pub(crate) use types::*;

use std::env;

{main_body}
"""
    MAIN.write_text(new_main, encoding="utf-8")
    print(f"main lines {new_main.count(chr(10)) + 1}")
    print(f"DONE modules={written} buckets={len(buckets)}")


if __name__ == "__main__":
    main()
