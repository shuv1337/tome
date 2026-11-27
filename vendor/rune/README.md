# Vendored Rune

`crates/rune` is [anomalyco/rune](https://github.com/anomalyco/rune) at
`7a7ed14e93ef00782a6c7c9a672608a9ba778cdb` (MIT, see `LICENSE`), overridden via
`[patch."https://github.com/anomalyco/rune"]` in the root `Cargo.toml`. Its
manifest expands inherited workspace settings and pins internal Rune
dependencies to that same revision. All other Rune crates (`rune-kit`,
`rune-fx`, internal `rune-*` crates) and the eight `[patch.crates-io]` entries
come from that git revision.

Tome enables `accessibility`, `dbus`, and `svg` (`default-features = false`,
matching upstream Strata: no `embedded-fonts`, `wayland-csd-title`,
`metal-backend`, `hunspell`, or `ogg-audio`).

## Local patches

1. **Wayland drag-source missing action fallback (upstream
   [anomalyco/rune#2639](https://github.com/anomalyco/rune/pull/2639), commit
   `8bee7158fbd9aada8e313a7eb6dad6797a0157ff`):** when `wl_data_source.dnd_finished`
   arrives without a prior `wl_data_source.action` event (as on Hyprland <=
   v0.57.2), `SourceAction::finished_outcome` falls back to `DragOutcome::Copied`
   when `Copy` (or `Copy | Move`) was offered, `DragOutcome::Moved` when only
   `Move` was offered, and `DragOutcome::Cancelled` when neither was offered.
   Drop this patch once the base revision includes `8bee7158f`.

## Adding or dropping a patch

1. Edit the files under `vendor/rune/crates/rune/` directly (keep changes
   minimal and well-scoped) and add or remove the entry in **Local patches**
   above.
2. Verify there is still a single `rune` crate in the dependency graph coming
   from `vendor/rune/crates/rune`:
   `cargo tree -i rune -e normal`
3. Run workspace checks and tests covering the patched area.

## Rebasing onto a new Rune rev

When merging an upstream Strata repin (`build(rune): repin rune to <rev>`) or
intentionally moving the base rev:

1. Check out the new `anomalyco/rune` revision in a scratch clone, re-apply any
   local patches from the list above that are not yet in that revision, and drop
   any that upstream now includes.
2. Replace `vendor/rune/crates/rune/` and `vendor/rune/LICENSE` from that tree.
3. Re-expand `workspace = true` fields in `vendor/rune/crates/rune/Cargo.toml`
   from the new revision's root `Cargo.toml`, pinning every internal `rune-*`
   and `procwatch` dependency with explicit `version =` (for `cargo-deny`'s
   `wildcards = "deny"` check) and `git = "https://github.com/anomalyco/rune"`
   (no `.git` suffix) at the new `rev`, and removing `[lints] workspace = true`.
4. Update the `rev =` entries in Tome's root `Cargo.toml` (`rune`, `rune-kit`,
   `rune-fx`, and `[patch.crates-io]`), this README, `mise.toml`, and
   `tests/container/Dockerfile` (if the Rust toolchain changed) together.
5. Refresh `Cargo.lock` and verify `cargo tree -i rune -e normal` resolves only
   the vendored `rune` crate.
