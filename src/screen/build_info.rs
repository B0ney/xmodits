//! Provide information about the application

use iced::widget::{column, container, text};
use iced::Element;

#[cfg(feature = "build_info")]
pub mod info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[cfg(feature = "build_info")]
pub fn view<'a, Message: 'a>() -> Option<Element<'a, Message>> {
    let info = [
        ("Version", info::PKG_VERSION),
        ("License", info::PKG_LICENSE),
        ("Repository", info::PKG_REPOSITORY),
        ("Git Hash", info::GIT_COMMIT_HASH.unwrap_or("none")),
        ("Rustc Version", info::RUSTC_VERSION),
        ("Target Architechture", info::CFG_TARGET_ARCH),
        ("Build Date", info::BUILT_TIME_UTC),
    ];

    let information = info.into_iter().fold(column![], |col, (label, value)| {
        col.push(text(format!("{label}: {value}")))
    });

    Some(container(information).into())
}

#[cfg(not(feature = "build_info"))]
pub fn view<'a, Message: 'a>() -> Option<Element<'a, Message>> {
    None
}
