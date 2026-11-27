// SPDX-License-Identifier: MIT

use rune::Color;

use crate::look::Look;
use crate::tokens::Token;

/// Window chrome slot tokens (header/pane buttons, active breadcrumb/tab text). `None` inherits.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Chrome {
    pub action_icon: Option<Color>,
    pub current_text: Option<Color>,
}

/// Passive chrome action icon colour: `chrome.action.icon`, else `kit.c.icon_accent`.
pub fn action_icon(look: &Look) -> Color {
    look.slots
        .chrome
        .action_icon
        .unwrap_or(look.kit.c.icon_accent)
}

/// Current location / active tab / pane heading text colour: `chrome.current.text`, else `kit.c.text_accent`.
pub fn current_text(look: &Look) -> Color {
    look.slots
        .chrome
        .current_text
        .unwrap_or(look.kit.c.text_accent)
}

pub const TOKENS: &[Token] = &[
    Token::color("chrome.action.icon", action_icon, |l, v| {
        l.slots.chrome.action_icon = Some(v)
    }),
    Token::color("chrome.current.text", current_text, |l, v| {
        l.slots.chrome.current_text = Some(v)
    }),
];
