use iced_wgpu::Renderer;
use iced_winit::{
    button, slider, Align, Button, Color, Column, Command, Element, Length,
    Program, Row, Slider, Text,
};

#[derive(Debug, Default)]
pub struct Controls {
    background_color: Color,
    sliders: [slider::State; 3],
    action_center: [button::State; 3],
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
    ShowAction,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            background_color: Color::WHITE,
            sliders: Default::default(),
            action_center: Default::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::ShowAction => {
                println!("Application show");
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let [r, g, b] = &mut self.sliders;
        let [b1, b2, b3] = &mut self.action_center;
        let background_color = self.background_color;

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::End)
            .push(
                Button::new(b1, Text::new("KOOMPI"))
                    .on_press(Message::ShowAction)
                    .width(Length::Units(100))
                    .height(Length::Fill),
            )
            .push(
                Button::new(b2, Text::new("SMALLWORLD"))
                    .on_press(Message::ShowAction)
                    .width(Length::Units(100))
                    .height(Length::Fill),
            )
            .push(
                Button::new(
                    b3,
                    Text::new("SELENDRA").color(Color::from_rgb8(255, 0, 100)),
                )
                .on_press(Message::ShowAction)
                .width(Length::Units(100))
                .height(Length::Fill),
            )
            .into()
    }
}
