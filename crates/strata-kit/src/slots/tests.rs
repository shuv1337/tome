// SPDX-License-Identifier: MIT

use crate::look::Look;
use crate::tokens::{self, Value};

/// Each slot token that inherits from a core token, with that token.
const SEEDS: &[(&str, &str)] = &[
    ("collection.row.radius", "radius.md"),
    ("columns.header.height", "frame.bar"),
    ("list.row.height", "row.list"),
    ("sidebar.row.height", "row.sidebar"),
    ("tabs.tab.height", "frame.bar"),
];

fn length(look: &Look, name: &str) -> f32 {
    match tokens::get(look, name) {
        Ok(Value::Length(x)) => x,
        other => panic!("{name} read as {other:?}"),
    }
}

#[test]
fn an_unset_slot_token_follows_its_source() {
    for &(slot, source) in SEEDS {
        let mut look = Look::fallback(false);
        let before = length(&look, slot);
        let raised = length(&look, source) + 5.0;
        tokens::set(&mut look, source, Value::Length(raised)).expect("set");
        assert_eq!(length(&look, slot), before + 5.0, "{slot} follows {source}");
    }
}

#[test]
fn a_set_slot_token_stops_following_its_source() {
    for &(slot, source) in SEEDS {
        let mut look = Look::fallback(false);
        tokens::set(&mut look, slot, Value::Length(50.0)).expect("set");
        tokens::set(&mut look, source, Value::Length(10.0)).expect("set");
        assert_eq!(length(&look, slot), 50.0, "{slot} keeps its own value");
    }
}

#[test]
fn slot_token_names_are_the_slot_id_a_part_and_a_property() {
    for (id, slot) in super::TOKENS {
        for token in *slot {
            let segments: Vec<_> = token.name().split('.').collect();
            assert_eq!(segments.len(), 3, "{}", token.name());
            assert_eq!(segments[0], *id, "{} is in slot {id}", token.name());
        }
    }
}

#[test]
fn the_selected_frames_hover_is_its_fill_deepened_by_default() {
    use super::icons::{frame_selected_fill, frame_selected_hover_fill};

    let look = Look::fallback(false);
    let (fill, hover) = (frame_selected_fill(&look), frame_selected_hover_fill(&look));
    assert_eq!(hover, look.kit.alpha(fill, fill.a * 30.0 / 22.0));
    assert!(hover.a > fill.a, "{} over {}", hover.a, fill.a);
}

/// C13 review 1: the hover bubble's styling has tokens of its own (`.preview-video-bubble`).
#[test]
fn the_media_bubble_styling_is_overridable_by_name() {
    let look = Look::fallback(false);
    for name in [
        "media.bubble.fill",
        "media.bubble.border",
        "media.bubble.radius",
        "media.bubble.pad",
        "media.bubble.cell_fill",
        "media.bubble.cell_radius",
    ] {
        assert!(tokens::get(&look, name).is_ok(), "{name}");
    }
}

#[test]
fn scroll_hover_width_token_toggles_optional_hover_thickness() {
    let mut look = Look::fallback(false);
    assert_eq!(super::scroll::hover_width(&look), None);
    assert_eq!(length(&look, "scroll.bar.hover_width"), 0.0);

    tokens::set(&mut look, "scroll.bar.hover_width", Value::Length(14.0)).expect("set");
    assert_eq!(super::scroll::hover_width(&look), Some(14.0));
    assert_eq!(length(&look, "scroll.bar.hover_width"), 14.0);

    tokens::set(&mut look, "scroll.bar.hover_width", Value::Length(0.0)).expect("clear");
    assert_eq!(super::scroll::hover_width(&look), None);
}

#[test]
fn chrome_sidebar_and_collection_color_slots_default_and_override() {
    let mut look = Look::fallback(true);
    assert_eq!(super::chrome::action_icon(&look), look.kit.c.icon_accent);
    assert_eq!(super::chrome::current_text(&look), look.kit.c.text_accent);
    assert_eq!(super::sidebar::row_icon(&look), look.kit.c.icon_accent);
    assert_eq!(
        super::sidebar::selected_border(&look),
        rune::Color::TRANSPARENT
    );
    assert_eq!(super::collection::entry_icon(&look), look.kit.c.icon_accent);
    assert_eq!(
        super::collection::selected_fill(&look),
        look.kit.alpha(look.fill.highlight, 0.24)
    );
    assert_eq!(
        super::collection::selected_border(&look),
        rune::Color::TRANSPARENT
    );
    assert_eq!(super::collection::hover_fill(&look), look.kit.c.hover);
    assert_eq!(
        super::columns::header_inactive_title(&look),
        super::columns::fade(
            &look,
            look.kit.c.bg_accent,
            super::columns::header_title_opacity(&look),
        )
    );

    let custom = crate::theme::Rgba8::parse("#7c8aa0").expect("hex").color();
    for name in [
        "chrome.action.icon",
        "chrome.current.text",
        "sidebar.row.icon",
        "sidebar.selected.border",
        "collection.entry.icon",
        "collection.selected.fill",
        "collection.selected.border",
        "collection.hover.fill",
        "columns.header.inactive_title",
    ] {
        tokens::set(&mut look, name, Value::Color(custom)).expect("set color slot");
        assert_eq!(tokens::get(&look, name).expect("get"), Value::Color(custom));
    }
    assert_eq!(super::chrome::action_icon(&look), custom);
    assert_eq!(super::chrome::current_text(&look), custom);
    assert_eq!(super::sidebar::row_icon(&look), custom);
    assert_eq!(super::sidebar::selected_border(&look), custom);
    assert_eq!(super::collection::entry_icon(&look), custom);
    assert_eq!(super::collection::selected_fill(&look), custom);
    assert_eq!(super::collection::selected_border(&look), custom);
    assert_eq!(super::collection::hover_fill(&look), custom);
    assert_eq!(super::columns::header_inactive_title(&look), custom);
}
