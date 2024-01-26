#[cfg(windows)]
mod simple;

use crate::event;
use crate::font;
use crate::icon;
use crate::logger;
use crate::ripper;
// use crate::screen::config::custom_filters;
use crate::screen::about;
use crate::screen::config::name_preview;
use crate::screen::config::sample_naming;
use crate::screen::config::sample_ripping::{self, DESTINATION_BAR_ID};
use crate::screen::entry::Entries;
use crate::screen::ripping;
use crate::screen::sample_player;
use crate::screen::settings;
use crate::screen::tracker_info::{self, TrackerInfo};
use crate::theme;
use crate::utils::{files_dialog, folders_dialog};
use crate::widget::helpers::{action, text_icon, warning};
use crate::widget::{Collection, Container, Element};

use data::Config;
pub use ripping::RippingState;
use std::path::PathBuf;

use iced::multi_window::{self, Application};
use iced::widget::{button, checkbox, column, row, text, text_input, Space};
use iced::{window, Alignment, Command, Length, Size, Subscription};

const TITLE: &str = "XMODITS";
const WINDOW_SIZE: Size = Size::new(780.0, 720.0);

#[derive(Debug, Clone)]
pub enum Message {
    AboutPressed,
    About(about::Message),
    Add(Option<Vec<PathBuf>>),
    Cancel,
    Clear,
    ConfigPressed,
    // CustomFilter(custom_filters::Message),
    DeleteSelected,
    Event(event::Event),
    FileDialog,
    // FilterPressed,
    FolderDialog,
    FontLoaded(Result<(), iced::font::Error>),
    GeneralCfg(settings::Message),
    Ignore,
    InvertSelection,
    NamingCfg(sample_naming::Message),
    Open(String),
    PreviewSamples(PathBuf),
    Probe(usize),
    ProbeResult(TrackerInfo),
    RippingCfg(sample_ripping::Message),
    SamplePlayer(sample_player::Message),
    SaveConfig,
    SaveConfigResult(),
    SaveErrors,
    SaveErrorsResult(Result<PathBuf, String>),
    Select { index: usize, selected: bool },
    SelectAll(bool),
    SetState(RippingState),
    SetTheme,
    SettingsPressed,
    StartRipping,
    Subscription(ripper::Message),
    Shutdown,
    BadModules(logger::bad_modules::Added),
}

/// This is basically the configuration panel view.
#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Configure,
    // Filters,
    Settings,
    About,
}

/// XMODITS graphical application
#[derive(Default)]
pub struct XMODITS {
    entries: Entries,
    state: RippingState,
    view: View,
    file_hovered: bool,
    ripper: ripper::Handle,
    tracker_info: TrackerInfo,
    sample_player: sample_player::SamplePreview,
    naming_cfg: data::config::SampleNameConfig,
    ripping_cfg: data::config::SampleRippingConfig,
    general_cfg: data::config::GeneralConfig,
    bad_modules: Vec<PathBuf>,
    // custom_filters: custom_filters::CustomFilters,
}

impl XMODITS {
    /// Launch the application
    pub fn launch() -> iced::Result {
        // load configuration
        let config = Config::load();

        //
        tracing::info!("Launcing GUI");
        Self::run(Self::settings(config))
    }

    /// WINDOWS ONLY
    ///
    /// XMODITS' simple mode to allow dragging and dropping modules onto the binary
    #[cfg(windows)]
    pub fn launch_simple(paths: impl IntoIterator<Item = String>) -> iced::Result {
        simple::rip(paths);
        Ok(())
    }

    pub fn settings(config: Config) -> iced::Settings<Config> {
        iced::Settings {
            default_font: font::JETBRAINS_MONO,
            default_text_size: 13.0.into(),
            flags: config,
            window: window::Settings {
                icon: Some(application_icon()),
                size: WINDOW_SIZE,
                min_size: Some(WINDOW_SIZE),
                ..Default::default()
            },
            antialiasing: true,
            ..Default::default()
        }
    }

    // todo
    pub fn load_cfg(&mut self, config: Config) {
        self.ripping_cfg = config.ripping;
        self.naming_cfg = config.naming;
        self.general_cfg = config.general;
    }

    pub fn build_start_signal(&mut self) -> ripper::Signal {
        self.tracker_info.clear();
        let entries = self.entries.take();
        let ripping = self.ripping_cfg.to_owned();
        let naming = self.naming_cfg.to_owned();

        ripper::Signal {
            entries,
            ripping,
            naming,
        }
    }

    pub fn clear_entries(&mut self) {
        if !self.state.is_ripping() {
            self.tracker_info.clear();
            self.entries.clear();
        } else if self.state.is_finished() {
            self.state = RippingState::Idle;
        }
    }

    pub fn delete_selected_entries(&mut self) {
        if !self.state.is_ripping() {
            self.entries.delete_selected(&mut self.tracker_info);
        }
    }

    pub fn app_title(&self) -> String {
        match &self.state {
            RippingState::Idle | RippingState::Finished { .. } => TITLE.to_string(),
            RippingState::Ripping {
                message, progress, ..
            } => {
                let info = format!(
                    "{} - {}%",
                    message.as_deref().unwrap_or("Ripping..."),
                    progress.floor()
                );
                format!("{TITLE} - {info}")
            }
        }
    }

    pub fn save_cfg(&self) -> Command<Message> {
        let config = data::Config {
            general: self.general_cfg.clone(),
            ripping: self.ripping_cfg.clone(),
            naming: self.naming_cfg,
        };

        Command::perform(async move { config.save().await }, |_| Message::Ignore)
    }

    pub fn start_ripping(&mut self) -> Command<Message> {
        if self.state.is_ripping() | self.entries.is_empty() | !self.ripper.is_active() {
            return Command::none();
        }

        if !sample_ripping::destination_is_valid(&self.ripping_cfg) {
            tracing::error!(
                "The provided destination is not valid. The *parent* folder must exist."
            );
            return text_input::focus(DESTINATION_BAR_ID.clone());
        }

        let start_signal = self.build_start_signal();
        self.ripper
            .send(start_signal)
            .expect("Sending start signal to Ripper.");

        self.state = RippingState::Ripping {
            message: None,
            progress: 0.0,
            errors: 0,
        };

        Command::none()
    }

    fn add_entry(&mut self, path: PathBuf) {
        self.add_entries(Some(vec![path]))
    }

    fn add_entries(&mut self, paths: Option<Vec<PathBuf>>) {
        if self.state.is_ripping() {
            return;
        }

        let Some(paths) = paths else { return };

        self.entries.add_multiple(paths);

        if self.state.is_finished() {
            self.state = RippingState::Idle;
        }
    }
}

/// TODO: allow the user to customize their application icon
pub fn application_icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logos/icon.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

impl multi_window::Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let mut app = Self::default();
        app.load_cfg(flags);

        (app, font::load().map(Message::FontLoaded))
    }

    fn title(&self, id: window::Id) -> String {
        match id == window::Id::MAIN {
            true => self.app_title(),
            false => self.sample_player.get_title(id),
        }
    }

    fn theme(&self, _id: window::Id) -> Self::Theme {
        theme::Theme(self.general_cfg.theme.palette()).clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AboutPressed => self.view = View::About,
            Message::ConfigPressed => self.view = View::Configure,
            // Message::FilterPressed => self.view = View::Filters,
            Message::SettingsPressed => self.view = View::Settings,
            Message::Add(paths) => self.add_entries(paths),
            Message::Clear => self.clear_entries(),
            Message::DeleteSelected => self.delete_selected_entries(),
            Message::InvertSelection => self.entries.invert(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::SelectAll(selected) => self.entries.select_all(selected),
            Message::SetState(state) => self.state = state,
            Message::About(msg) => return about::update(msg).map(Message::About),
            Message::FileDialog => {
                return Command::perform(files_dialog(), Message::Add);
            }
            Message::FolderDialog => {
                return Command::perform(folders_dialog(), Message::Add);
            }
            Message::GeneralCfg(cfg) => {
                return settings::update(&mut self.general_cfg, cfg).map(Message::GeneralCfg)
            }
            Message::RippingCfg(msg) => {
                return sample_ripping::update(&mut self.ripping_cfg, msg).map(Message::RippingCfg)
            }
            Message::NamingCfg(msg) => sample_naming::update(&mut self.naming_cfg, msg),
            // Message::CustomFilter(msg) => return self.custom_filters.update(msg).map(Message::CustomFilter),
            Message::SetTheme => todo!(),
            Message::Open(link) => {
                if let Err(err) = open::that_detached(link) {
                    tracing::warn!("Could not open external link: {:?}", err)
                };
            }
            Message::PreviewSamples(path) => {
                return self
                    .sample_player
                    .create_instance(path)
                    .map(Message::SamplePlayer);
            }
            Message::Probe(idx) => {
                let path = self.entries.get(idx).unwrap();

                if self.tracker_info.matches_path(path) | path.is_dir() {
                    return Command::none();
                }

                return Command::perform(
                    tracker_info::probe(path.to_owned()),
                    Message::ProbeResult,
                );
            }
            Message::ProbeResult(probe) => self.tracker_info = probe,
            Message::SamplePlayer(msg) => {
                return self
                    .sample_player
                    .update(msg, &mut self.entries)
                    .map(Message::SamplePlayer)
            }
            Message::SaveConfig => {
                return self.save_cfg();
            }
            Message::SaveConfigResult() => {}
            Message::SaveErrors => return self.state.export_errors(),
            Message::SaveErrorsResult(result) => {
                if let Ok(path) = result {
                    tracing::info!("Successfully saved errors to: {}", &path.display());
                    if self.general_cfg.show_errors_in_text_editor {
                        let _ = open::that_detached(path);
                    }
                }
            }
            Message::StartRipping => {
                return self.start_ripping();
            }
            Message::Cancel => {
                self.state.set_message("Cancelling...");
                self.ripper.cancel();
            }
            Message::Event(event) => match event {
                event::Event::Clear => self.clear_entries(),
                event::Event::Closed(id) => match id != window::Id::MAIN {
                    true => self.sample_player.remove_instance(id),
                    false => return self.sample_player.close_all().map(Message::SamplePlayer),
                },
                event::Event::CloseRequested => {}
                event::Event::Delete => self.delete_selected_entries(),
                event::Event::FileHoveredLeft(id) => match id == window::Id::MAIN {
                    true => self.file_hovered = false,
                    false => self.sample_player.set_hovered(id, false),
                },
                event::Event::FileHovered(id, _) => match id == window::Id::MAIN {
                    true => self.file_hovered = true,
                    false => self.sample_player.set_hovered(id, true),
                },
                event::Event::FileDropped(id, file) => {
                    if id == window::Id::MAIN {
                        self.add_entry(file);
                        self.file_hovered = false;
                    } else {
                        self.sample_player.set_hovered(id, false);
                        return self
                            .sample_player
                            .load_samples(id, file)
                            .map(Message::SamplePlayer);
                    }
                }
                event::Event::Save => return self.save_cfg(),
                event::Event::Start => return self.start_ripping(),
            },
            Message::Subscription(msg) => match msg {
                ripper::Message::Ready(sender) => self.ripper.set_sender(sender),
                ripper::Message::Info(info) => self.state.update_message(info),
                ripper::Message::Progress { progress, errors } => {
                    self.state.update_progress(progress, errors)
                }
                ripper::Message::Done {
                    state,
                    time,
                    destination,
                } => {
                    self.state = RippingState::Finished {
                        state,
                        time,
                        destination,
                    };
                }
            },
            Message::Ignore => (),
            Message::FontLoaded(result) => {
                if let Err(e) = result {
                    tracing::error!("Failed to load font: {:#?}", e);
                }
            }
            Message::Shutdown => return window::close(window::Id::MAIN),
            Message::BadModules(file) => {
                tracing::error!(
                    "This module might have caused the fatal error: {}",
                    file.path.display()
                );
                self.bad_modules.push(file.path);
            }
        }
        Command::none()
    }

    fn view(&self, _id: window::Id) -> Element<Message> {
        #[cfg(feature = "audio")]
        if _id > window::Id::MAIN {
            return self
                .sample_player
                .view(_id, &self.entries)
                .map(Message::SamplePlayer);
        }

        let top_left_menu = row![
            button("Ripping").on_press(Message::ConfigPressed),
            // button("Filters").on_press(Message::FilterPressed),
            button("Settings").on_press(Message::SettingsPressed),
            button("About").on_press(Message::AboutPressed),
        ]
        .spacing(5)
        .width(Length::Fill)
        .align_items(Alignment::Center);

        let not_ripping = !self.state.is_ripping();

        let bottom_left_buttons = row![
            button(text_icon("Save Settings", icon::save()))
                .on_press(Message::SaveConfig)
                .width(Length::FillPortion(2))
                .padding(8),
            button(text_icon("START", icon::download()))
                .on_press_maybe(not_ripping.then_some(Message::StartRipping))
                .style(theme::Button::Start)
                .width(Length::FillPortion(2))
                .padding(8)
        ]
        .spacing(8);

        let left_view = match self.view {
            View::Configure => {
                let naming_cfg = {
                    let name_preview = name_preview::preview_name(
                        &self.general_cfg.sample_name_params,
                        &self.naming_cfg,
                        &self.ripping_cfg,
                    );

                    sample_naming::view(&self.naming_cfg, name_preview).map(Message::NamingCfg)
                };

                let ripping_cfg = sample_ripping::view(&self.ripping_cfg).map(Message::RippingCfg);

                column![
                    self.tracker_info.view(),
                    naming_cfg,
                    ripping_cfg,
                    bottom_left_buttons,
                ]
                .spacing(10)
                .into()
            }
            // View::Filters => column![
            //     self.custom_filters.view_file_size().map(Message::CustomFilter),
            //     self.custom_filters.view_file_date().map(Message::CustomFilter),
            //     self.custom_filters.view_file_name().map(Message::CustomFilter),
            //     bottom_left_buttons,
            // ]
            // .spacing(10)
            // .into(),
            View::Settings => settings::view(&self.general_cfg).map(Message::GeneralCfg),
            View::About => about::view().map(Message::About),
        };

        let left_half = column![top_left_menu, Space::with_height(2), left_view]
            .width(Length::FillPortion(4))
            .spacing(10);

        let destination =
            sample_ripping::view_destination_bar(&self.ripping_cfg).map(Message::RippingCfg);

        let top_right_buttons = row![
            text(format!(
                "Entries: {}, Selected: {}",
                self.entries.len(),
                self.entries.total_selected()
            )),
            Space::with_width(Length::Fill),
            button("Invert").on_press(Message::InvertSelection),
            checkbox(
                "Select All",
                self.entries.all_selected(),
                Message::SelectAll
            )
            .style(theme::CheckBox::Inverted)
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let bottom_right_buttons = row![
            action("Add File", not_ripping.then_some(Message::FileDialog)).padding(8),
            action("Add Folder", not_ripping.then_some(Message::FolderDialog)).padding(8),
            Space::with_width(Length::Fill)
        ]
        .push_maybe(
            (self.entries.total_selected() > 0 && (!self.entries.all_selected())).then(|| {
                action(
                    "Clear Selected",
                    not_ripping.then_some(Message::DeleteSelected),
                )
                .padding(8)
                .style(theme::Button::Cancel)
            }),
        )
        .push(
            action("Clear", not_ripping.then_some(Message::Clear))
                .padding(8)
                .style(theme::Button::Cancel),
        )
        .spacing(5);

        let show_gif = !self.general_cfg.hide_gif;
        let main_view = match &self.state {
            RippingState::Idle => self.entries.view(self.file_hovered, show_gif),
            RippingState::Ripping {
                message,
                progress,
                errors,
            } => ripping::view_ripping(message, *progress, *errors, show_gif),
            RippingState::Finished {
                state,
                time,
                destination,
            } => ripping::view_finished(
                state,
                time,
                self.file_hovered,
                destination,
                &self.bad_modules,
            ),
        };

        let allow_warnings = !self.general_cfg.suppress_warnings;

        let bad_cfg_warning = warning(
            || allow_warnings && (!self.ripping_cfg.self_contained && !self.naming_cfg.prefix),
            "\"Self Contained\" is disabled. You should enable \"Prefix Samples\" to reduce collisions. Unless you know what you are doing."
        );

        let too_many_files_warning = warning(
            || allow_warnings && self.entries.len() > 200,
            "That's a lot of files! You REALLY should be using folders.",
        );

        let right_half = column![destination, top_right_buttons, main_view]
            .push_maybe(bad_cfg_warning)
            .push_maybe(too_many_files_warning)
            .push(bottom_right_buttons)
            .width(Length::FillPortion(5))
            .spacing(10);

        let main = row![left_half, right_half].spacing(10);

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch([
            event::events().map(Message::Event),
            ripper::subscription().map(Message::Subscription),
            logger::bad_modules::subscription().map(Message::BadModules),
        ])
    }
}
