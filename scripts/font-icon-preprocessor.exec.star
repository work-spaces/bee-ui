#!/usr/bin/env spaces
"""
Generate Rust source files for Bootstrap icons and embedded font bytes.

This script scans:
  runnerd/crates/runnerd-bs/assets/icons
  runnerd/crates/runnerd-bs/assets/fonts/{mono,propo}

Usage:
  runnerd/scripts/font-icon-preprocessor.exec.star \
    --input-folder <icons_dir> \
    --icon-output <icons_rs_path> \
    --font-output <fonts_rs_path>
"""

load("//@star/prelude/exec/args.star", "args_opt", "args_parse", "args_parser")
load("//@star/prelude/exec/fs.star", "fs_exists", "fs_is_directory", "fs_mkdir", "fs_walk_directory", "fs_write_text")
load("//@star/prelude/exec/io.star", "io_eprint", "io_print")
load("//@star/prelude/exec/path.star", "path_dirname")
load("//@star/prelude/exec/string.star", "string_lower", "string_replace", "string_split_whitespace", "string_upper")
load("//@star/prelude/exec/sys.star", "sys_exit")

DEFAULT_FONTS_DIR = "runnerd/crates/runnerd-bs/assets/fonts"
ICON_ASSET_PREFIX = "/assets/icons/"
FONT_ASSET_PREFIX = "/assets/fonts/"

def _is_ascii_digit(ch: str) -> bool:
    return ch >= "0" and ch <= "9"

def _to_rust_variant(raw_id: str) -> str:
    # Convert path/name separators and punctuation to token boundaries.
    normalized = string_replace(raw_id, "[^A-Za-z0-9]+", " ", regex = True)
    tokens = string_split_whitespace(normalized)

    parts = []
    for token in tokens:
        lower = string_lower(token)
        if not lower:
            continue

        if _is_ascii_digit(lower[0]):
            # Enum variants cannot start with a digit.
            parts.append("N" + lower)
        else:
            parts.append(string_upper(lower[:1]) + lower[1:])

    variant = "".join(parts)
    if not variant:
        fail("Could not derive a Rust enum variant for id: {}".format(raw_id))

    return variant

def _to_rust_variant_preserve_case(raw_id: str) -> str:
    # Like _to_rust_variant, but preserves existing casing inside each token.
    normalized = string_replace(raw_id, "[^A-Za-z0-9]+", " ", regex = True)
    tokens = string_split_whitespace(normalized)

    parts = []
    for token in tokens:
        if not token:
            continue

        if _is_ascii_digit(token[0]):
            parts.append("N" + token)
        else:
            parts.append(string_upper(token[:1]) + token[1:])

    variant = "".join(parts)
    if not variant:
        fail("Could not derive a Rust enum variant for id: {}".format(raw_id))

    return variant

def _assert_directory_exists(path: str, label: str):
    if not fs_exists(path):
        io_eprint("error: {} does not exist: {}".format(label, path))
        fail("failed")
    if not fs_is_directory(path):
        io_eprint("error: {} is not a directory: {}".format(label, path))
        fail("failed")

def _collect_icon_paths(icons_dir: str) -> list[str]:
    _assert_directory_exists(icons_dir, "Icons directory")

    def pick_svg(entry):
        if entry["is_file"] and string_lower(entry["name"]).endswith(".svg"):
            return string_replace(entry["relative_path"], "\\", "/")
        return None

    svg_paths = fs_walk_directory(
        path = icons_dir,
        callback = pick_svg,
        recursive = True,
        include_files = True,
        include_dirs = False,
    )

    svg_paths = sorted(svg_paths)
    if not svg_paths:
        fail("No SVG files found under: {}".format(icons_dir))

    return svg_paths

def _collect_font_paths(fonts_dir: str) -> dict:
    _assert_directory_exists(fonts_dir, "Fonts directory")

    def pick_font(entry):
        if not entry["is_file"]:
            return None

        rel_path = string_replace(entry["relative_path"], "\\", "/")
        lower_rel_path = string_lower(rel_path)
        is_font = (
            lower_rel_path.endswith(".ttf") or
            lower_rel_path.endswith(".otf") or
            lower_rel_path.endswith(".woff") or
            lower_rel_path.endswith(".woff2")
        )
        if not is_font:
            return None

        if rel_path.startswith("mono/") or rel_path.startswith("propo/"):
            return rel_path

        return None

    font_paths = fs_walk_directory(
        path = fonts_dir,
        callback = pick_font,
        recursive = True,
        include_files = True,
        include_dirs = False,
    )

    font_paths = sorted(font_paths)
    if not font_paths:
        fail("No font files found under: {}".format(fonts_dir))

    mono_paths = []
    propo_paths = []
    for rel_path in font_paths:
        if rel_path.startswith("mono/"):
            mono_paths.append(rel_path)
        elif rel_path.startswith("propo/"):
            propo_paths.append(rel_path)

    if not mono_paths:
        fail("No font files found under: {}/mono".format(fonts_dir))
    if not propo_paths:
        fail("No font files found under: {}/propo".format(fonts_dir))

    return {
        "mono": mono_paths,
        "propo": propo_paths,
    }

def _build_icon_entries(svg_paths: list[str]) -> list[dict]:
    entries = []
    seen_variants = {}

    for rel_path in svg_paths:
        icon_id = rel_path[:-4]  # trim .svg
        variant = _to_rust_variant(icon_id)

        if variant in seen_variants:
            fail("Variant collision for '{}': '{}' and '{}'".format(
                variant,
                seen_variants[variant],
                rel_path,
            ))

        seen_variants[variant] = rel_path
        entries.append({
            "variant": variant,
            "icon_id": icon_id,
            "rel_path": rel_path,
        })

    return entries

def _strip_extension(path: str) -> str:
    return string_replace(path, "\\.[^.]+$", "", regex = True)

def _shared_prefix(values: list[str]) -> str:
    if not values:
        return ""

    prefix = values[0]
    for value in values[1:]:
        matched = ""
        limit = len(prefix)
        if len(value) < limit:
            limit = len(value)

        for idx in range(limit, -1, -1):
            candidate = prefix[:idx]
            if value.startswith(candidate):
                matched = candidate
                break

        prefix = matched
        if not prefix:
            break

    return prefix

def _split_font_id(font_id: str) -> dict:
    base_name = string_replace(font_id, "^.*/", "", regex = True)

    if "-" not in base_name:
        return {
            "font_stem": base_name,
            "variation_id": base_name,
        }

    return {
        "font_stem": string_replace(base_name, "-.*$", "", regex = True),
        "variation_id": string_replace(base_name, "^[^-]+-", "", regex = True),
    }

def _build_font_entries(font_paths: list[str], family: str) -> list[dict]:
    raw_entries = []
    seen_stems = {}
    stems = []
    family_prefix = family + "/"

    for rel_path in font_paths:
        family_rel_path = rel_path[len(family_prefix):]
        font_id = _strip_extension(family_rel_path)
        name_parts = _split_font_id(font_id)

        font_stem = name_parts["font_stem"]
        if font_stem not in seen_stems:
            seen_stems[font_stem] = True
            stems.append(font_stem)

        raw_entries.append({
            "font_id": font_id,
            "font_stem": font_stem,
            "variation_id": name_parts["variation_id"],
            "rel_path": rel_path,
        })

    shared_stem_prefix = _shared_prefix(stems)
    has_multiple_stems = len(stems) > 1

    entries = []
    seen_variants = {}

    for entry in raw_entries:
        variant_id = entry["variation_id"]

        if has_multiple_stems:
            stem_suffix = entry["font_stem"]
            if shared_stem_prefix and stem_suffix.startswith(shared_stem_prefix):
                stem_suffix = stem_suffix[len(shared_stem_prefix):]
            if stem_suffix:
                variant_id = stem_suffix + "-" + variant_id

        variant = _to_rust_variant_preserve_case(variant_id)

        if variant in seen_variants:
            fail("Variant collision in {} fonts for '{}': '{}' and '{}'".format(
                family,
                variant,
                seen_variants[variant],
                entry["rel_path"],
            ))

        seen_variants[variant] = entry["rel_path"]
        entries.append({
            "variant": variant,
            "font_id": entry["font_id"],
            "rel_path": entry["rel_path"],
        })

    return entries

def _render_icon_rust(entries: list[dict]) -> str:
    lines = []
    lines.append("// @generated by runnerd/scripts/font-icon-preprocessor.exec.star")
    lines.append("// DO NOT EDIT MANUALLY.")
    lines.append("")
    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")
    lines.append("pub enum Icon {")
    for entry in entries:
        lines.append("    {},".format(entry["variant"]))
    lines.append("}")
    lines.append("")
    lines.append("impl Icon {")
    lines.append("    pub const ALL: &'static [Self] = &[")
    for entry in entries:
        lines.append("        Self::{},".format(entry["variant"]))
    lines.append("    ];")
    lines.append("")
    lines.append("    pub fn svg(self) -> &'static str {")
    lines.append("        match self {")
    for entry in entries:
        lines.append('            Self::{} => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "{}{}")),'.format(
            entry["variant"],
            ICON_ASSET_PREFIX,
            entry["rel_path"],
        ))
    lines.append("        }")
    lines.append("    }")
    lines.append("}")

    return "\n".join(lines) + "\n"

def _render_font_family(lines: list[str], enum_name: str, entries: list[dict]):
    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")
    lines.append("pub enum {} {{".format(enum_name))
    for entry in entries:
        lines.append("    {},".format(entry["variant"]))
    lines.append("}")
    lines.append("")
    lines.append("impl {} {{".format(enum_name))
    lines.append("    #[allow(dead_code)]")
    lines.append("    pub const ALL: &'static [Self] = &[")
    for entry in entries:
        lines.append("        Self::{},".format(entry["variant"]))
    lines.append("    ];")
    lines.append("")
    lines.append("    pub fn bytes(self) -> &'static [u8] {")
    lines.append("        match self {")
    for entry in entries:
        lines.append('            Self::{} => include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "{}{}")),'.format(
            entry["variant"],
            FONT_ASSET_PREFIX,
            entry["rel_path"],
        ))
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")

def _render_font_rust(mono_entries: list[dict], propo_entries: list[dict]) -> str:
    lines = []
    lines.append("// @generated by runnerd/scripts/font-icon-preprocessor.exec.star")
    lines.append("// DO NOT EDIT MANUALLY.")
    lines.append("")

    _render_font_family(lines, "Mono", mono_entries)
    _render_font_family(lines, "Propo", propo_entries)

    lines.append("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")
    lines.append("pub enum Font {")
    lines.append("    Mono(Mono),")
    lines.append("    Propo(Propo),")
    lines.append("}")
    lines.append("")
    lines.append("pub fn include_font(font: Font) -> &'static [u8] {")
    lines.append("    match font {")
    lines.append("        Font::Mono(font) => font.bytes(),")
    lines.append("        Font::Propo(font) => font.bytes(),")
    lines.append("    }")
    lines.append("}")

    return "\n".join(lines) + "\n"

def _make_parser() -> dict:
    return args_parser(
        name = "font-icon-preprocessor.exec.star",
        description = "Generate Rust icon and font modules with embedded assets",
        options = [
            args_opt(
                "--icon-folder",
                help = "Input directory containing icon SVG files",
            ),
            args_opt(
                "--font-folder",
                help = "Input directory containing font mono/propo folders",
            ),
            args_opt(
                "--icon-output",
                help = "Path to generated Rust icon source file",
            ),
            args_opt(
                "--font-output",
                help = "Path to generated Rust font source file",
            ),
        ],
    )

def _ensure_parent_directory(path: str):
    output_dir = path_dirname(path)
    if output_dir:
        fs_mkdir(output_dir, parents = True, exist_ok = True)

def main() -> int:
    parsed = args_parse(_make_parser())
    icon_input_folder = parsed.get("icon_folder", "")
    font_input_folder = parsed.get("font_folder", "")
    icon_output_path = parsed.get("icon_output", "")
    font_output_path = parsed.get("font_output", "")

    if not icon_input_folder:
        io_eprint("missing required --icon-input-folder argument")
        return 2

    if not font_input_folder:
        io_eprint("missing required --font-input-folder argument")
        return 2

    if not icon_output_path:
        io_eprint("missing required --icon-output argument")
        return 2

    if not font_output_path:
        io_eprint("missing required --font-output argument")
        return 2

    icon_paths = _collect_icon_paths(icon_input_folder)
    icon_entries = _build_icon_entries(icon_paths)
    icon_source = _render_icon_rust(icon_entries)

    font_paths = _collect_font_paths(font_input_folder)
    mono_entries = _build_font_entries(font_paths["mono"], "mono")
    propo_entries = _build_font_entries(font_paths["propo"], "propo")
    font_source = _render_font_rust(mono_entries, propo_entries)

    _ensure_parent_directory(icon_output_path)
    _ensure_parent_directory(font_output_path)

    fs_write_text(icon_output_path, icon_source)
    fs_write_text(font_output_path, font_source)

    io_print("generated {} icon variants into {}".format(len(icon_entries), icon_output_path))
    io_print(
        "generated {} mono + {} propo font variants into {}".format(
            len(mono_entries),
            len(propo_entries),
            font_output_path,
        ),
    )
    return 0

sys_exit(main())
