/*!
A binding for `SDL2_ttf`.
 */

#[allow(non_camel_case_types, dead_code)]
mod ffi;
mod font;
mod context;

// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_ttf")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_ttf")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_ttf")]
    extern {}
}

pub use self::context::{
    init, has_been_initialized, get_linked_version, Sdl2TtfContext, InitError,
};
pub use self::font::{
    Font, FontStyle, Hinting, GlyphMetrics, PartialRendering, FontError,
    FontResult, STYLE_NORMAL, STYLE_BOLD, STYLE_ITALIC, STYLE_UNDERLINE, STYLE_STRIKETHROUGH
};
