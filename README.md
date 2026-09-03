# pixi-build-rust cross-compile CFLAGS leak - minimal reproducer

Cross-compiling `linux-aarch64 -> linux-64` with `pixi-build-rust` (aarch64
runner producing an x86-64 package) fails with:

```
cc1: error: unknown value 'nocona' for '-march'
```

## Root cause

conda-forge's cross-compilation activation scripts scope env vars by
convention: unsuffixed `CC`/`CFLAGS` are for HOST (the platform being built
*for*), `CC_<triple>`/`CFLAGS_<triple>` are for BUILD (the machine actually
running the compiler - used for proc-macros and build scripts, which always
run natively regardless of the overall cross-compilation target).

On x86-64, HOST's generic `CFLAGS` includes `-march=nocona -mtune=haswell`.
On aarch64, HOST's generic `CFLAGS` sets no `-march=`/`-mtune=` at all - this
is why the bug is only observed in the aarch64-runner -> linux-64 direction
and not the reverse.

`cc-rs` (the `cc` crate) resolves `CC`/`AR` with first-match-wins across
`CC_<triple>`, but resolves `CFLAGS` by *concatenating* every matching
variable - `CFLAGS_<triple>` does not override generic `CFLAGS`, it is
appended to. So a native dependency that is correctly compiled for BUILD
(clean `CFLAGS_<triple>`) still receives HOST's `-march=nocona`, which its
own (BUILD-platplatform) compiler rejects.

## Shape of this repro

```
pixi-cc-leak-repro (bin)
 └─ proc-macro-crate (proc-macro = true, always compiled for BUILD)
     └─ native-dep-crate (build.rs calls `cc::Build::new().compile(..)`)
```

This is the same shape as the real-world trigger
(`git-cliff-core -> include-flate-codegen -> zstd-sys`), stripped to the
essential mechanism: a proc-macro pulling in a native build-dependency.

## Reproducing

Needs an aarch64 machine building for `linux-64`. `.github/workflows/repro.yml`
runs it on GitHub's free `ubuntu-24.04-arm` runners:

- `aarch64-runner-build-linux-64` - expected to **fail** with the `nocona` error.
- `aarch64-runner-build-linux-aarch64` - expected to **pass** (BUILD == HOST,
  nothing to leak), kept side by side to show the asymmetry.

Locally, on an aarch64 machine:

```bash
pixi build --target-platform=linux-64
```
