# npm releases

Use the staging helper in the repo root to generate npm tarballs for a release. For
example, to stage the CLI and SDK packages for version `0.6.0`:

```bash
./scripts/stage_npm_packages.py \
  --release-version 0.6.0 \
  --package mova \
  --package mova-sdk
```

This downloads the required native package archive artifacts, hydrates `vendor/` for
each package, and writes tarballs to `dist/npm/`.

To stage only Apple Silicon macOS artifacts for a gradual rollout, add the
target filter:

```bash
./scripts/stage_npm_packages.py \
  --release-version 0.6.0 \
  --target aarch64-apple-darwin \
  --package mova \
  --package mova-sdk
```

When `--package mova` is provided, the staging helper builds the lightweight
`@movscript/mova` meta package plus the selected platform-native
`@movscript/mova` variants that are later published under platform-specific
dist-tags. Without `--target`, every release target is selected.

Direct `build_npm_package.py` invocations are still useful for package-specific
debugging, but native packages expect `--vendor-src` to point at a prehydrated
`vendor/` tree. Release packaging should use `scripts/stage_npm_packages.py`.
