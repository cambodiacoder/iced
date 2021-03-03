use crate::styles::{buttonstyle, containers};
use buttonstyle::buttons::ButtonStyle::Transparent as btnzero;
use iced::Text;
use iced_wgpu::Renderer;
use iced_winit::winit::event_loop::ControlFlow;
use iced_winit::{
    button, slider, Align, Button, Color, Column, Command, Container, Element,
    Font, HorizontalAlignment, Length, Program, Row, Slider, Space,
};
#[derive(Debug)]
pub struct Controls {
    background_color: Color,
    action_center: [button::State; 7],
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
    ShowMenu,
    MonitorShow,
    BellShow,
    KeyboardShow,
    ClipboardShow,
    SoundShow,
    WifiShow,
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            background_color: Color::WHITE,
            action_center: Default::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl<'a> Program for Controls {
    type Renderer = Renderer;
    type Message = Message;
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::ShowMenu => {
                println!("Menu show");
                // let window = Window::new(&self.event).unwrap();
                // window.set_window_icon(window_icon: Option<Icon>)
                // let window = Window::new();
            }
            Message::MonitorShow => {}
            Message::BellShow => {}
            Message::ClipboardShow => {}
            Message::SoundShow => {}
            Message::WifiShow => {}
            Message::KeyboardShow => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let [b1, b2, b3, b4, b5, b6, b7] = &mut self.action_center;
        // let background_color = self.background_color;
        let menu = Button::new(b1, menu_icon())
            .style(btnzero)
            .on_press(Message::ShowMenu)
            .width(Length::Shrink)
            .height(Length::Shrink);
        let system_tray = Row::new()
            .align_items(Align::Center)
            .push(
                Button::new(b2, monitor_icon())
                    .style(btnzero)
                    .on_press(Message::MonitorShow),
            )
            .push(
                Button::new(b3, bell_icon())
                    .style(btnzero)
                    .on_press(Message::BellShow),
            )
            .push(
                Button::new(b4, keyboard_icon())
                    .style(btnzero)
                    .on_press(Message::KeyboardShow),
            )
            .push(
                Button::new(b5, clipboard())
                    .style(btnzero)
                    .on_press(Message::ClipboardShow),
            )
            .push(
                Button::new(b6, sound_icon())
                    .style(btnzero)
                    .on_press(Message::SoundShow),
            )
            .push(
                Button::new(b7, wifi_icon())
                    .style(btnzero)
                    .on_press(Message::WifiShow),
            );
        let row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::End)
            .push(menu)
            .push(Space::with_width(Length::Fill))
            .push(system_tray);
        Container::new(row)
            .style(containers::CustomContainer::ForegroundWhite)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn menu_icon() -> Text {
    icon('\u{f0c9}')
}
fn monitor_icon() -> Text {
    icon('\u{f108}')
}
fn bell_icon() -> Text {
    icon('\u{f0f3}')
}
fn clipboard() -> Text {
    icon('\u{f328}')
}
fn keyboard_icon() -> Text {
    icon('\u{f11c}')
}
fn sound_icon() -> Text {
    icon('\u{f028}')
}
fn wifi_icon() -> Text {
    icon('\u{f1eb}')
}
fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}
const ICONS: Font = Font::External {
    name: "Line Awesome",
    bytes: include_bytes!("./font/la-solid-900.ttf"),
};

#[derive(Debug, Default)]
pub struct WidgetState {
    icon: char,
    state: button::State,
}
