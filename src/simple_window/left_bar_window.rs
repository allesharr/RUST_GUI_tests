use iced::{widget::{button, Container, Column, Row, Button}, Application, Length, Alignment, Theme, Element, Command, Renderer, Settings};

#[derive(Debug, Clone, Copy)]
enum Message {
    ToggleMenu,
    Save,
    Settings,
    About,
}

struct LeftBarApp {
    menu_button: button::State,
    is_menu_visible: bool,
}

impl Application for LeftBarApp {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new(flags: Self::Flags) -> (LeftBarApp, Command<Message>) {
        (
            LeftBarApp {
                menu_button: button::State::new(),
                is_menu_visible: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced App with Menu")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleMenu => {
                self.is_menu_visible = !self.is_menu_visible;
            }
            Message::Save => {
                // Handle "Save" button click
                println!("Save button clicked!");
            }
            Message::Settings => {
                // Handle "Settings" button click
                println!("Settings button clicked!");
            }
            Message::About => {
                // Handle "About" button click
                println!("About button clicked!");
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let about_button:Button<'_, Message> = button("About")
            .on_press(Message::About);
        let settings_button:Button<'_, Message> = button("About")
            .on_press(Message::Settings);
        let save_button:Button<'_, Message> = button("About")
            .on_press(Message::Save);

            let empty_about_button:Button<'_, Message> = button("About")
            .on_press(Message::About);

        let menu_content = Column::new()
            .spacing(20)
            .push(about_button)
            .push(settings_button)
            .push(save_button);

        let empty_content = Column::new()
        .spacing(20).
        align_items(Alignment::Center)
        .push(empty_about_button);


        let menu = Container::new(menu_content)
            .width(Length::FillPortion(200))
            .padding(20)
            .center_x()
            .center_y();

        let empty_menu = Container::new(empty_content)
        .width(Length::FillPortion(200))
        .padding(20)
        .center_x()
        .center_y();


            let menu_button = button("Show menu")
            .on_press(Message::ToggleMenu);

            let content = Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(menu_button)
            .push(
                if self.is_menu_visible {
                    menu
                } else {
                  empty_menu
                });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    type Theme = Theme;

    type Flags = ();

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn style(&self) -> <Self::Theme as iced::application::StyleSheet>::Style {
        <Self::Theme as iced::application::StyleSheet>::Style::default()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

}


pub fn left_bar_window_start() {
    LeftBarApp::run(Settings::default());
}