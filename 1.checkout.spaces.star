"""
Spaces starlark checkout/run script to make changes to spaces, printer, and archiver.
With VSCode/Zed integration
"""

load("//@star/packages/star/package.star", "package_add")
load("//@star/packages/star/python.star", "python_add_uv")
load("//@star/packages/star/rust.star", "rust_add")
load("//@star/packages/star/spaces-cli.star", "spaces_add_devutils", "spaces_add_star_formatter")
load("//@star/packages/star/starship.star", "starship_add_bash")
load(
    "//@star/prelude/info.star",
    "info_is_ci",
)
load(
    "//@star/prelude/rules/checkout.star",
    "checkout_add_archive",
    "checkout_add_env_vars",
    "checkout_add_exec",
)
load("//@star/prelude/rules/env.star", "env_inherit")
load(
    "//@star/prelude/rules/sdk.star",
    "sdk_is_owner",
)

# Configure the top level workspace

_ICON_PATH = "bee-ui/crates/bee-ui/assets/icons"
_FONT_PATH = "bee-ui/crates/bee-ui/assets/fonts"

checkout_add_archive(
    "bootstrap_icons_archive",
    url = "https://github.com/twbs/icons/releases/download/v1.13.1/bootstrap-icons-1.13.1.zip",
    sha256 = "999021e12fab5c9ede5e4e7072eb176122be798b2f99195acf5dda47aef8fc93",
    add_prefix = _ICON_PATH,
    strip_prefix = "bootstrap-icons-1.13.1",
    includes = [
        "bootstrap-icons-1.13.1/*.svg",
    ],
)

# https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip
#
checkout_add_archive(
    "mono_font_archive",
    url = "https://download.jetbrains.com/fonts/JetBrainsMono-2.304.zip",
    sha256 = "6f6376c6ed2960ea8a963cd7387ec9d76e3f629125bc33d1fdcd7eb7012f7bbf",
    add_prefix = _FONT_PATH + "/mono",
    strip_prefix = "fonts/ttf",
    includes = [
        "fonts/ttf/**",
    ],
    excludes = [
        "fonts/ttf/**/*NL-*.ttf",
    ],
)

checkout_add_archive(
    "sans_font_archive",
    url = "https://github.com/IBM/plex/releases/download/%40ibm%2Fplex-sans-condensed%402.0.0/ibm-plex-sans-condensed.zip",
    sha256 = "f86ba73abbeed7cf7690ead863192a87211dfc2b0cef0ac6bf4700533ec5ab9e",
    add_prefix = _FONT_PATH + "/propo",
    strip_prefix = "ibm-plex-sans-condensed/fonts/complete/ttf",
    includes = ["ibm-plex-sans-condensed/fonts/complete/ttf/*.ttf"],
)

if sdk_is_owner():
    spaces_add_devutils(
        "spaces0",
        "v0.21.0",
        devutils_version = "devutils-v0.1.15",
        system_paths = ["/usr/bin", "/bin"],
    )

    spaces_add_star_formatter("star_formatter", configure_zed = True, deps = [":spaces0"])

    if not info_is_ci():
        shortcuts = {
        }
        starship_add_bash(
            "starship0",
            shortcuts = shortcuts,
            install_binary = False,
            deps = [":spaces0"],
        )

    rust_add(
        "rust_toolchain",
        version = "1.93",
        deps = [":spaces0"],
    )

cargo_binstall1 = package_add("github.com", "cargo-bins", "cargo-binstall", "v1.21.1")

checkout_add_exec(
    "wasm_pack",
    command = "cargo-binstall",
    args = ["--no-confirm", "wasm-pack"],
    deps = [
        ":rust_toolchain",
        cargo_binstall1,
    ],
)

checkout_add_env_vars(
    "spaces_env",
    vars = [
        env_inherit(
            "GH_TOKEN",
            is_secret = True,
            is_required = info_is_ci(),
            help = "Allows access to gh in Spaces rules",
        ),
    ],
)
