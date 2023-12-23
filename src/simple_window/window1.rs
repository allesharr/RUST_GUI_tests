use iced::{
    widget::button, Application, widget::Column, Command, widget::Container, Element, Length, Settings, widget::Text,
};
use iced::Theme;

use crate::counter::counter;

#[derive(Debug, Clone)]
enum Screen {
    Home,
    OtherWindow,
    AnotherWindow,
}

#[derive(Debug, Clone)]
enum Message {
    SwitchScreen(Screen),
}

struct Test_App{
    current_screen: Screen,
    switch_button: button::State,
}

impl Application for Test_App{
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Test_App {
                current_screen: Screen::Home,
                switch_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match self.current_screen {
            Screen::Home => String::from("Home"),
            Screen::OtherWindow => String::from("Other Window"),
            Screen::AnotherWindow => String::from("Another Widow")
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchScreen(screen) => {
                self.current_screen = screen;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.current_screen {
            Screen::Home => {
                let switch_button = button("Open Home Screen")
                    .on_press(Message::SwitchScreen(Screen::OtherWindow));

                Column::new()
                    .spacing(20)
                    .push(Text::new("Home Screen"))
                    .push(switch_button)

                
            }
            Screen::OtherWindow => {
                let switch_button = button("pen other screen")
                    .on_press(Message::SwitchScreen(Screen::Home));

                Column::new()
                    .spacing(20)
                    .push(Text::new("Other Window"))
                    .push(switch_button)
            }

            Screen::AnotherWindow => {

               let switch_button = button("Press me")
               .on_press(Message::SwitchScreen((Screen::Home)));
                    Column::new()
                    .spacing(20)
                    .push(Text::new("Another Window"))
                    .push(switch_button)
            }
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}


pub fn window1_start() {
    Test_App::run(Settings::default());
}
