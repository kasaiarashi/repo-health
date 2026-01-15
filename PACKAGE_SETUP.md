# Package Manager Setup

This document explains how to set up repo-health for various package managers.

## Chocolatey

To publish repo-health to Chocolatey:

1. **Update version** in `chocolatey/repo-health.nuspec`
2. **Update URL and checksum** in `chocolatey/tools/chocolateyinstall.ps1`:
   ```powershell
   # Get checksum
   certutil -hashfile repo-health-windows-x64.exe SHA256
   ```
3. **Package**:
   ```powershell
   cd chocolatey
   choco pack
   ```
4. **Test locally**:
   ```powershell
   choco install repo-health -source .
   ```
5. **Publish**:
   ```powershell
   choco push repo-health.0.1.0.nupkg --source https://push.chocolatey.org/
   ```

**Note**: You need a Chocolatey API key. Get one from https://community.chocolatey.org/account

## Scoop

To make repo-health available via Scoop, you need to create a Scoop bucket:

1. **Create a GitHub repository** named `scoop-bucket`

2. **Add the manifest** (`repo-health.json`) to the repository root

3. **Update the hash**:
   ```powershell
   # Get hash
   (Get-FileHash repo-health-windows-x64.exe -Algorithm SHA256).Hash
   ```

4. **Users can then install with**:
   ```powershell
   scoop bucket add kasaiarashi https://github.com/kasaiarashi/scoop-bucket
   scoop install repo-health
   ```

### Auto-updating Scoop manifest

The manifest includes `checkver` and `autoupdate` sections that allow Scoop to automatically detect new versions:

```json
{
  "checkver": {
    "github": "https://github.com/kasaiarashi/repo-health"
  },
  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/kasaiarashi/repo-health/releases/download/v$version/repo-health-windows-x64.exe"
      }
    }
  }
}
```

## Cargo

Publishing to crates.io:

1. **Login**:
   ```bash
   cargo login
   ```

2. **Publish**:
   ```bash
   cargo publish
   ```

Users can then install with:
```bash
cargo install repo-health
```

## GitHub Releases

The GitHub Actions workflow (`.github/workflows/release.yml`) automatically:
- Builds binaries for Windows and Linux
- Creates a GitHub release when you push a version tag
- Uploads binaries as release assets

To create a new release:
```bash
git tag v0.2.0
git push origin v0.2.0
```

## Homebrew (Future)

For macOS support via Homebrew:

1. Create a Homebrew tap repository
2. Add a formula file
3. Users install with: `brew install kasaiarashi/tap/repo-health`
