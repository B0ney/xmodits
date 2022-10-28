use iced::{
    button,
    slider, Column, Element, ProgressBar, Sandbox, Settings, Slider, Container, Length, Text, Button, Alignment};

pub fn launch() {
    let _ = Progress::run(Settings::default());
    // println!("GUI application launced!")
}


#[derive(Default)]
struct Progress{
    value: f32,
    progress_bar_slider: slider::State,
    show_confirm: bool,
    exit: bool,
    confirm_button: button::State,
    exit_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Exit,
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
                Button::new(
                    &mut self.confirm_button,
                    Text::new("Yes, exit now"),
                )
                .padding([10, 20])
                .on_press(Message::Confirm),
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