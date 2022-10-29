use iced::{
    button, slider, window::Icon, Alignment, Button, Column, Container, Element, Length,
    ProgressBar, Sandbox, Settings, Slider, Text, window
};
use image::{self, GenericImageView};
use std::fs::File;
use std::io::Cursor;
use std::{thread, io::BufReader, sync::Arc};
use crossbeam_channel::{unbounded, Sender};

use rodio::source::{Buffered, Source};
use rodio::Decoder;

type SoundSource = Buffered<Decoder<BufReader<File>>>;

pub fn launch() {
    // image::Image::
    let image = image::load_from_memory(include_bytes!("../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    let icon = Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap();

    

    // std::thread::sleep(std::time::Duration::from_secs(3));
    

    let _ = Progress::run(Settings {
        window: window::Settings {
            size: (400, 600),
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
    tick_button: button::State,
    exit_button: button::State,
    sender: Option<Sender<Msg>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
    Cancel,
    SliderChanged(f32),
    Sfx,

}
enum Msg {
    Play
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        let (tx, rx) = unbounded::<Msg>();
        std::thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            // let f = include_bytes!("../res/sfx/confirm.wav");
            let f = include_bytes!("../res/sfx/acimalaka.ogg");

            let a = Cursor::new(f);
            let source = Decoder::new(a).unwrap().buffered();

            loop {
                if let Ok(msg) = rx.recv() {
                    match msg {
                        Msg::Play => {
                            sink.append(source.clone());
                            // sink.sleep_until_end();
                        },
                    }
                }
            }
        });
        Self {sender: Some(tx), ..Default::default()}
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
            Message::Sfx => self.sender.as_ref().unwrap().send(Msg::Play).unwrap(),
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
                        .on_press(Message::Cancel)
                        // .on_press(Message::Cancel),
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
                .push(
                    Button::new(&mut self.tick_button, Text::new("tick"))
                        .padding([10, 20])
                        .on_press(Message::Sfx),
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
