use iced::{
    button, slider, window::Icon, Alignment, Button, Column, Container, Element, Length,
    ProgressBar, Sandbox, Settings, Slider, Text, window
};
use image::{self, GenericImageView};
use std::thread;

pub fn launch() {
    // image::Image::
    let image = image::load_from_memory(include_bytes!("../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    let icon = Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap();


    let _ = Progress::run(Settings {
        window: window::Settings {
            size: (800, 400),
            // 
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    });
    // println!("GUI application launced!")
}

pub struct Application {}

#[derive(Default)]
struct Progress {
    value: f32,
    progress_bar_slider: slider::State,
    show_confirm: bool,
    exit: bool,
    confirm_button: button::State,
    cancel_button: button::State,
    exit_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
    Cancel,
    SliderChanged(f32),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
            Message::Confirm => {
                self.exit = true;
            }
            Message::Exit => {
                self.show_confirm = true;
            }
            Message::Cancel => {
                self.show_confirm = false;
            }
        }
    }
    fn should_exit(&self) -> bool {
        self.exit
    }

    fn view(&mut self) -> Element<Message> {
        let content = if self.show_confirm {
            Column::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(Text::new("Are you sure you want to exit?"))
                .push(
                    Button::new(&mut self.confirm_button, Text::new("Yes, exit now"))
                        .padding([10, 20])
                        .on_press(Message::Confirm),
                )
                .push(
                    Button::new(&mut self.cancel_button, Text::new("Nah, go back"))
                        .padding([10, 20])
                        .on_press(Message::Cancel),
                )
        } else {
            Column::new()
                .padding(20)
                .push(ProgressBar::new(0.0..=100.0, self.value))
                .push(
                    Slider::new(
                        &mut self.progress_bar_slider,
                        0.0..=100.0,
                        self.value,
                        Message::SliderChanged,
                    )
                    .step(0.01),
                )
                .push(Text::new("Click the button to exit"))
                .push(
                    Button::new(&mut self.exit_button, Text::new("Exit"))
                        .padding([10, 20])
                        .on_press(Message::Exit),
                )
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}
