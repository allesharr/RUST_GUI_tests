use iced::{Theme, Application};
use iced::{widget::Column, widget::Container, Element, Length, widget::Slider, widget::Text};
use iced::widget::slider;

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f32),
}

struct LeftSliderExample {
    slider_value: f32,
}

impl iced::Application for LeftSliderExample {

    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    type Theme = Theme;



    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            LeftSliderExample { slider_value: 0.5},
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Left Slider")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SliderChanged(value) => {
                self.slider_value = value;
                iced::Command::none()
            }
        }

    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
 

        let slider = Slider::new(
            0.0..=100.0,
            self.slider_value,
            Message::SliderChanged
        )
        .step(0.01)
        .width(iced::Length::Fill);


        let content = Column::new()
            .push(iced::widget::Text::new("Left Slider Example"))
            .push(slider);

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .into()

    }


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

pub fn left_Slider_window_start() {
    LeftSliderExample::run(iced::Settings::default());
}


#[cfg(test)]
mod tests{
    use iced::Application;

    use super::LeftSliderExample;

    #[test]
    fn start_app() {
        LeftSliderExample::run(iced::Settings::default());
    }
}