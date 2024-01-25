//! The soul of XMODITS

pub mod extraction;
pub mod handle;
pub mod signal;
pub mod stop_flag;
pub mod subscription;
pub mod bad_modules;

pub use extraction::strict_loading;
pub use handle::Handle;
pub use signal::Signal;
pub use subscription::Message as RippingMessage;
pub use bad_modules::Added;

#[derive(Debug, Clone)]
pub enum Message {
    Ripper(subscription::Message),
    BadModule(bad_modules::Added)
}

pub fn subscription() -> iced::Subscription<Message> {
    iced::Subscription::batch([
        subscription::xmodits_subscription().map(Message::Ripper),
        bad_modules::subscription().map(Message::BadModule)
    ])
}