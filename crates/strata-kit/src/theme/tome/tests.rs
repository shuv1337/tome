// SPDX-License-Identifier: MIT

use super::*;
use crate::shape::LookParams;
use crate::slots::{chrome, collection, columns, icons, sidebar};
use crate::theme::{bundled_theme, parse_custom};
use crate::tokens::{self, Kind};

#[test]
fn s2_dark_tokens_parse_cleanly_and_set_only_colours() {
    let overrides = s2_dark_overrides();
    assert!(overrides.errors().is_empty(), "{:?}", overrides.errors());
    let base = bundled_theme("s2-dark").expect("s2-dark is bundled");
    let look = base.look();
    let plain = bundled_theme("tokyo-night")
        .expect("tokyo-night is bundled")
        .look();
    for token in tokens::all().filter(|t| t.kind() != Kind::Color) {
        assert_eq!(token.get(&look), token.get(&plain), "{}", token.name());
    }
}

#[test]
fn s2_dark_refinements_apply_and_survive_with_params() {
    let theme = bundled_theme("s2-dark").expect("s2-dark is bundled");
    let look = theme.look().with_params(&LookParams::default());
    assert_eq!(
        collection::entry_icon(&look),
        crate::theme::Rgba8::parse("#7c8aa0").expect("hex").color()
    );
    assert_eq!(
        sidebar::row_icon(&look),
        crate::theme::Rgba8::parse("#7c8aa0").expect("hex").color()
    );
    assert_eq!(
        sidebar::selected_fill(&look),
        crate::theme::Rgba8::parse("#1d2635").expect("hex").color()
    );
    assert_eq!(
        sidebar::selected_border(&look),
        crate::theme::Rgba8::parse("#2a364a").expect("hex").color()
    );
    assert_eq!(
        collection::selected_fill(&look),
        crate::theme::Rgba8::parse("#1d2635").expect("hex").color()
    );
    assert_eq!(
        collection::selected_border(&look),
        crate::theme::Rgba8::parse("#2a364a").expect("hex").color()
    );
    assert_eq!(
        collection::active_path_fill(&look),
        crate::theme::Rgba8::parse("#1d2635").expect("hex").color()
    );
    assert_eq!(
        collection::active_parent_fill(&look),
        crate::theme::Rgba8::parse("#161d28").expect("hex").color()
    );
    assert_eq!(
        columns::header_inactive_title(&look),
        crate::theme::Rgba8::parse("#94a3b8").expect("hex").color()
    );
    assert_eq!(
        icons::frame_icon(&look),
        crate::theme::Rgba8::parse("#7c8aa0").expect("hex").color()
    );
    assert_eq!(
        chrome::action_icon(&look),
        crate::theme::Rgba8::parse("#94a3b8").expect("hex").color()
    );
    assert_eq!(
        chrome::current_text(&look),
        crate::theme::Rgba8::parse("#e6ebf2").expect("hex").color()
    );
    assert_eq!(
        look.kit.c.surface_raised,
        crate::theme::Rgba8::parse("#161d28").expect("hex").color()
    );
    assert_eq!(
        look.kit.c.border_focus,
        crate::theme::Rgba8::parse("#6366f1").expect("hex").color()
    );
    assert_eq!(
        icons::frame_focus_ring(&look),
        crate::theme::Rgba8::parse("#6366f1").expect("hex").color()
    );
    assert_eq!(
        icons::frame_selected_hover_fill(&look),
        crate::theme::Rgba8::parse("#242f40").expect("hex").color()
    );
    assert_eq!(icons::caption_hover_fill(&look), look.kit.c.hover);
    assert_eq!(look.kit.c.surface_hover, look.kit.c.hover);
    assert_eq!(look.kit.c.surface_raised_hover, look.kit.c.hover);
    let kit = theme.kit();
    assert_eq!(kit.c.surface_raised, look.kit.c.surface_raised);
    assert_eq!(kit.c.border_focus, look.kit.c.border_focus);
    assert_eq!(kit.c.hover, look.kit.c.hover);
    assert_eq!(kit.c.surface_hover, look.kit.c.surface_hover);
}

#[test]
fn a_custom_theme_reusing_s2_dark_id_is_not_refined() {
    let src = r##"name = "Custom S2"
background = "#0c1119"
surface = "#121923"
text = "#e6ebf2"
accent = "#7c83ff"
danger = "#f87171"
muted = "#1b232f"
highlight = "#1d2635"
border = "#242f40"
dim_text = "#94a3b8"
"##;
    let custom = parse_custom("s2-dark", src).expect("loads");
    let look = custom.look();
    assert_eq!(collection::entry_icon(&look), look.kit.c.icon_accent);
    assert_eq!(chrome::action_icon(&look), look.kit.c.icon_accent);
    assert_eq!(sidebar::selected_border(&look), rune::Color::TRANSPARENT);
}
