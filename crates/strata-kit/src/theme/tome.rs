// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use super::colors::{Source, Theme};
use crate::look::Look;
use crate::overrides::Overrides;
use crate::transparency::BaseSurfaces;

#[cfg(test)]
mod tests;

pub(super) const CATALOG: &str = include_str!("../../themes/tome/catalog.toml");

const S2_DARK_TOKENS: &str = include_str!("../../themes/tome/s2-dark.tokens.toml");

fn s2_dark_overrides() -> &'static Overrides {
    static OVERRIDES: OnceLock<Overrides> = OnceLock::new();
    OVERRIDES.get_or_init(|| {
        let parsed = Overrides::parse(S2_DARK_TOKENS);
        assert!(
            parsed.errors().is_empty(),
            "themes/tome/s2-dark.tokens.toml: {:?}",
            parsed.errors()
        );
        parsed
    })
}

/// Applies Tome's bundled token refinements to `look`, leaving upstream, custom, and Omarchy themes untouched.
pub(super) fn refine(theme: &Theme, look: &mut Look) {
    if theme.source != Source::Bundled {
        return;
    }
    let Some(overrides) = (match theme.id.as_str() {
        "s2-dark" => Some(s2_dark_overrides()),
        _ => None,
    }) else {
        return;
    };
    overrides.apply(look);
    look.base_surfaces = BaseSurfaces::from_palette(&look.kit.c);
}
