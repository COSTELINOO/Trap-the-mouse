#![warn(unused_variables)]
use iced::{
    widget::canvas::{self, Frame, Geometry, Path}, Point, Rectangle, Renderer, Size,
};
use iced::{Theme, Border, Shadow};
use iced::{Color, Background};
use iced::widget::button::{Style, Status};
use iced::border::Radius;
use iced::widget::{container, text_input};

#[derive(Debug, Clone)]
pub enum ShapeType {
    Rectangle { x: f32, y: f32, width: f32, height: f32, culoare: Color },
    Trapezoid { x: f32, y: f32, top_width: f32, bottom_width: f32, height: f32, culoare: Color },
}


pub struct ShapeCollection {
    pub shape: ShapeType,
    cache: canvas::Cache,
}

impl ShapeCollection {
    pub fn new(shape: ShapeType) -> Self {
        ShapeCollection {
            shape,
            cache: canvas::Cache::new(),
        }
    }

}

impl<Message> canvas::Program<Message> for ShapeCollection {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        viewport: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(viewport, bounds.size(), |frame: &mut Frame| {
            match &self.shape {
                ShapeType::Rectangle { x, y, width, height, culoare } => {
                    let rect = Path::rectangle(Point::new(*x, *y), Size::new(*width, *height));
                    frame.fill(&rect, *culoare);
                }


                ShapeType::Trapezoid { x, y, top_width, bottom_width, height, culoare } => {
                    let trapezoid = create_trapezoid(*x, *y, *top_width, *bottom_width, *height);
                    frame.fill(&trapezoid, *culoare);
                }
            }
        });

        vec![geometry]
    }

    fn mouse_interaction(
        &self,
        _: &Self::State,
        layout: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        if let Some(cursor_position) = cursor.position() {
            if cursor_position.x >= layout.x
                && cursor_position.x <= layout.x + layout.width
                && cursor_position.y >= layout.y
                && cursor_position.y <= layout.y + layout.height
            {
                iced::mouse::Interaction::Pointer
            } else {
                iced::mouse::Interaction::Idle
            }
        } else {
            iced::mouse::Interaction::Idle
        }
    }
}


fn create_trapezoid(x: f32, y: f32, top_width: f32, bottom_width: f32, height: f32) -> Path {
    Path::new(|builder| {
        let top_left = Point::new(x - top_width / 2.0, y);
        let top_right = Point::new(x + top_width / 2.0, y);
        let bottom_left = Point::new(x - bottom_width / 2.0, y + height);
        let bottom_right = Point::new(x + bottom_width / 2.0, y + height);
        builder.move_to(top_left);
        builder.line_to(top_right);
        builder.line_to(bottom_right);
        builder.line_to(bottom_left);
        builder.close();
    })
}


#[derive(Debug, Clone)]
pub struct Tema
{
    pub colors: [String; 4],
}

pub fn set_tema(index: u32, set_tema: &mut Tema)
{
    match index {
        1 => set_tema.colors = ["0233336".to_string(), "4da674".to_string(), "c1e6ba".to_string(), "eaf8e7".to_string()],

        2 => set_tema.colors = ["4233736".to_string(), "987185".to_string(), "d6aa9f".to_string(), "f4e2d1".to_string()],

        3 => set_tema.colors = ["112032".to_string(), "3e4d5f".to_string(), "aab7b7".to_string(), "c0c8ca".to_string()],

        4 => set_tema.colors = ["2a3517".to_string(), "8f9c77".to_string(), "cfe1b9".to_string(), "e7f5dc".to_string()],

        5 => set_tema.colors = ["291c0e".to_string(), "6e473b".to_string(), "a78d78".to_string(), "beb5a9".to_string()],

        _ => set_tema.colors = ["4c1d3d".to_string(), "dc586d".to_string(), "fb9590".to_string(), "dcbcaa".to_string()],
    }
}

pub fn culori(index: usize, tema: &Tema) -> Color {
    let fallback = "ffffff".to_string();
    let hex = tema.colors.get(index).unwrap_or(&fallback);
    Color::parse(hex).unwrap_or(Color::BLACK)
}

pub fn button_style(theme: &Theme, status: Status, tema: &Tema, culoare: Color) -> Style {
    match theme
    {
        Theme::Light
        => {
            match status {
                Status::Active => Style {
                    background: Some(Background::Color(culori(1, tema))),

                    text_color: culori(0, tema),

                    border: Border {
                        color: culori(0, tema),
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 3.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    },
                },

                Status::Hovered => Style {
                    background: Some(Background::Color(culori(2, tema))),

                    text_color: culori(0, tema),

                    border: Border {
                        color: culori(1, tema),
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 4.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    },
                },

                Status::Pressed => Style {
                    background: Some(Background::Color(culori(1, tema))),

                    text_color: culori(0, tema),

                    border: Border {
                        color: culori(0, tema),
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 2.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    },
                },

                Status::Disabled => Style {
                    background: Some(Background::Color(culori(1, tema))),

                    text_color: culori(5, tema),

                    border: Border {
                        color: culori(0, tema),
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
            }
        }

        Theme::Dracula =>
            {
                Style {
                    background: Some(Background::Color(culori(2, tema))),

                    text_color: culori(0, tema),

                    border: Border {
                        color: culori(1, tema),
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 4.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    },

                }
            }

        Theme::Ferra =>
            {
                match status {
                    Status::Active => Style {
                        background: Some(Background::Color(Color::WHITE)),

                        text_color: Color::parse("11100f").unwrap(),

                        border: Border {
                            color: Color::parse("11100f").unwrap(),
                            width: 2.0,
                            radius: Radius::new(5.0),
                        },

                        shadow: Shadow {
                            offset: iced::Vector::new(0.0, 2.0),
                            blur_radius: 3.0,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        },
                    },

                    Status::Hovered => Style
                    {
                        background: Some(Background::Color(culori(2, tema))),

                        text_color: culori(0, tema),

                        border: Border {
                            color: culori(1, tema),
                            width: 2.0,
                            radius: Radius::new(5.0),
                        },

                        shadow: Shadow {
                            offset: iced::Vector::new(0.0, 4.0),
                            blur_radius: 4.0,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                        },
                    },
                    Status::Pressed => Style {
                        background: Some(Background::Color(culori(1, tema))),

                        text_color: culori(0, tema),

                        border: Border {
                            color: culori(0, tema),
                            width: 2.0,
                            radius: Radius::new(5.0),
                        },

                        shadow: Shadow {
                            offset: iced::Vector::new(0.0, 2.0),
                            blur_radius: 2.0,
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        },
                    },

                    Status::Disabled => Style {
                        background: Some(Background::Color(culori(1, tema))),

                        text_color: culori(5, tema),

                        border: Border {
                            color: culori(0, tema),
                            width: 2.0,
                            radius: Radius::new(5.0),
                        },

                        shadow: Shadow {
                            offset: iced::Vector::new(0.0, 0.0),
                            blur_radius: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                }
            }
        _ =>
            {
                Style {
                    background: Some(Background::Color(culoare)),

                    text_color: culori(0, tema),

                    border: Border {
                        color: culoare,
                        width: 5.0,
                        radius: Radius::new(5.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 0.0,
                        color: Color::TRANSPARENT,
                    },
                }
            }
    }
}


pub fn container_style(theme: &Theme, status: Status, tema: &Tema, culoare: Color) -> container::Style {
    match theme
    {
        Theme::Light
        => match status {
            Status::Active => container::Style {
                background: Some(Background::Color(culori(1, tema))),

                text_color: Some(culori(0, tema)),

                border: Border {
                    color: culori(0, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                shadow: Shadow {
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 3.0,
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                },
            },

            Status::Hovered => container::Style {
                background: Some(Background::Color(culori(2, tema))),

                text_color: Some(culori(0, tema)),

                border: Border {
                    color: culori(1, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                shadow: Shadow {
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 4.0,
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                },
            },

            _ => container::Style {
                background: Some(Background::Color(culori(1, tema))),

                text_color: Some(culori(0, tema)),

                border: Border {
                    color: culori(0, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                shadow: Shadow {
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 2.0,
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                },
            },
        }

        _ =>
            {
                container::Style {
                    background: Some(Background::Color(culoare)),

                    text_color: Some(culori(0, tema)),

                    border: Border {
                        color: culoare,
                        width: 5.0,
                        radius: Radius::new(50.0),
                    },

                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 0.0,
                        color: Color::TRANSPARENT,
                    },
                }
            }
    }
}

pub fn text_input_style(theme: &Theme, status: text_input::Status, tema: &Tema, culoare: Color) -> text_input::Style {
    match theme
    {
        Theme::Light
        => match status {
            text_input::Status::Active => text_input::Style {
                background: Background::Color(culori(1, tema)),

                border: Border {
                    color: culori(0, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                icon: Color::TRANSPARENT,

                placeholder: culori(0, tema),

                value: culori(0, tema),

                selection: culori(2, tema),
            },

            text_input::Status::Focused { is_hovered: _ } => text_input::Style {
                background: Background::Color(culori(2, tema)),

                border: Border {
                    color: culori(1, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                icon: Color::TRANSPARENT,

                placeholder: culori(0, tema),

                value: culori(0, tema),

                selection: culori(3, tema),
            },
            _ => text_input::Style {
                background: Background::Color(culori(1, tema)),

                border: Border {
                    color: culori(0, tema),
                    width: 2.0,
                    radius: Radius::new(5.0),
                },

                icon: Color::TRANSPARENT,
                placeholder: culori(0, tema),
                value: culori(0, tema),
                selection: culori(0, tema),
            },
        }

        _ =>
            {
                text_input::Style {
                    background: Background::Color(culoare),

                    border: Border {
                        color: culoare,
                        width: 5.0,
                        radius: Radius::new(50.0),
                    },

                    icon: Color::TRANSPARENT,

                    placeholder: culori(0, tema),

                    value: culori(0, tema),

                    selection: culori(0, tema),

                }
            }
    }
}
