# contrib

Packaging and integration files.

| Path | What |
|---|---|
| `install.sh` | Universal installer: detects platform, downloads the latest release, verifies the checksum, installs to `~/.local/bin`. |
| `npm/` | The `furcule` npm package: a thin wrapper whose postinstall downloads the release binary so `npx furcule mcp` works in MCP configs. Version must match a release tag. |
| `aur/PKGBUILD` | Arch Linux package, builds from the release tarball. |
| `homebrew/furcule.rb` | Formula for `y0sif/homebrew-tap`, binary install per platform. Fill in the sha256 values on each release. |
| `systemd/furcule.service` | User unit that keeps `furcule serve` running for a graphs directory. |

All of these depend on GitHub releases produced by `.github/workflows/release.yml`, whose archive names they share: `furcule-<tag>-<target>.tar.gz` and `.zip` on Windows.
