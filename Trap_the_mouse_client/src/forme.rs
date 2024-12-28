use crate::messages::Message;
use iced::{
    widget::canvas::{self, Canvas, Frame, Geometry, Path},
     Element, Length, Point, Rectangle, Renderer, Size,Task
};

use iced::advanced;
use iced::{  Theme,Border,Shadow};
use iced::Color;
use iced::Background;
use iced::widget::button::Status::{Active,Hovered as Hover,Disabled,Pressed};
use iced::widget::button::Style;
use iced::border::Radius;
use std::sync::Arc;
use iced::widget::button::Status;
use iced::widget::{button, container, text_input, Button};
use iced::widget::text::Catalog;
use iced::widget::text_input::Cursor;
#[derive(Debug, Clone)]
pub enum ShapeType {
    Rectangle { x: f32, y: f32, width: f32, height: f32,culoare: Color },
    Circle { x: f32, y: f32, radius: f32,culoare: Color },
    Hexagon { x: f32, y: f32, size: f32,culoare: Color },
    Trapezoid { x: f32, y: f32, top_width: f32, bottom_width: f32, height: f32,culoare: Color },
}


pub struct ShapeCollection {
    pub shape: ShapeType, // Un singur tip de forma
    cache: canvas::Cache,
}


impl ShapeCollection {

    // Modificam constructorul pentru a accepta doar un tip de forma
    pub fn new(shape: ShapeType) -> Self {
        ShapeCollection {
            shape,
            cache: canvas::Cache::new(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> iced::mouse::Interaction {
        // Obține coordonatele cursorului
        if let Some(cursor_position) = cursor.position() {
            // Verifică dacă cursorul este deasupra butonului
            if layout.bounds().contains(cursor_position) {
                iced::mouse::Interaction::Pointer // Pointer indică că utilizatorul poate interacționa
            } else {
                iced::mouse::Interaction::Idle // Idle pentru starea implicită
            }
        } else {
            iced::mouse::Interaction::Idle // Idle dacă poziția cursorului este necunoscută
        }
    }

}

impl<Message> canvas::Program<Message> for ShapeCollection {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        viewport: &Renderer,
        _cursor: &iced::Theme,
        bounds: Rectangle,
        _cursor_position: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(viewport, bounds.size(), |frame: &mut Frame| {
            match &self.shape {
                ShapeType::Rectangle { x, y, width, height,culoare } => {
                    let rect = Path::rectangle(Point::new(*x, *y), Size::new(*width, *height));
                    frame.fill(&rect, *culoare);
                }
                ShapeType::Circle { x, y, radius,culoare } => {
                    let circle = Path::circle(Point::new(*x, *y), *radius);
                    frame.fill(&circle, *culoare);
                }
                ShapeType::Hexagon { x, y, size,culoare } => {
                    let hexagon = create_hexagon(*x, *y, *size);
                    frame.fill(&hexagon, *culoare);
                }
                ShapeType::Trapezoid { x, y, top_width, bottom_width, height,culoare} => {
                    let trapezoid = create_trapezoid(*x, *y, *top_width, *bottom_width, *height);
                    frame.fill(&trapezoid, *culoare);
                }
            }
        });

        vec![geometry]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        layout: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        if let Some(cursor_position) = cursor.position() {
            // Verifică dacă cursorul este deasupra formei
            if cursor_position.x >= layout.x
                && cursor_position.x <= layout.x + layout.width
                && cursor_position.y >= layout.y
                && cursor_position.y <= layout.y + layout.height
            {
                iced::mouse::Interaction::Pointer // Cursorul este deasupra formei
            } else {
                iced::mouse::Interaction::Idle // Cursorul nu este deasupra formei
            }
        } else {
            iced::mouse::Interaction::Idle // Dacă nu se știe poziția cursorului
        }
    }


}

fn create_hexagon(x: f32, y: f32, size: f32) -> Path {
    Path::new(|builder| {
        let angle = std::f32::consts::PI / 3.0;
        builder.move_to(Point::new(
            x + size * (0.0_f32).cos(),
            y + size * (0.0_f32).sin(),
        ));
        for i in 1..6 {
            let theta = i as f32 * angle;
            builder.line_to(Point::new(x + size * theta.cos(), y + size * theta.sin()));
        }
        builder.close();
    })
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

//const COLORS: [&str; 6] = ["11100f", "5d1c34", "a67d44", "2171b5", "cdbcab", "efe9e1"];


//EMERALD
//const COLORS: [&str; 6] = ["0233336", "4da674", "c1e6ba", "eaf8e7", "cdbcab", "efe9e1"];

//CANDY
//const COLORS: [&str; 6] = ["f6e8df","3e3c6e", "fe979b", "feae97", "efe9e1", "efe9e1"];

//AQUA
//const COLORS: [&str; 6] = ["50adbf","193841", "5d1c34","2171b5" ,"e95354" , "efe9e1"];

//Sniper
//const COLORS: [&str; 6] = ["1b1b1b","77abb6", "f1ac20","023336" ,"e95354" , "efe9e1"];

//Princess
//const COLORS: [&str; 6] = ["5d1c34","48a0a3", "9cbbfc","d9eff7" ,"e95354" , "efe9e1"];
#[derive(Debug,Clone)]
pub struct Tema
{
   pub COLORS: [String; 4]
}

pub fn set_tema(index: u32,set_tema : & mut Tema )
{
    match index {
        //EMERALD
        1 =>set_tema.COLORS= ["0233336".to_string(), "4da674".to_string(), "c1e6ba".to_string(), "eaf8e7".to_string()],
        //CANDY
        2 =>set_tema.COLORS= [ "4233736".to_string(),"987185".to_string(), "d6aa9f".to_string(),"f4e2d1".to_string()],
        //AQUA
        3 =>set_tema.COLORS= [ "112032".to_string(),"3e4d5f".to_string(), "aab7b7".to_string(),"c0c8ca".to_string()],
        //DEFAULT
        4 =>set_tema.COLORS= [ "2a3517".to_string(),"8f9c77".to_string(), "cfe1b9".to_string(),"e7f5dc".to_string()],
        //Coffee
        5=> set_tema.COLORS=["291c0e".to_string(),"6e473b".to_string(), "a78d78".to_string(),"beb5a9".to_string()],
        _ =>set_tema.COLORS= ["4c1d3d".to_string(),"dc586d".to_string(), "fb9590".to_string(),"dcbcaa".to_string()],
    }
}

pub fn culori(index: usize, tema: &Tema) -> Color {
    let fallback = "ffffff".to_string(); // Alb ca fallback
    let hex = tema.COLORS.get(index).unwrap_or(&fallback); // Folosim `.get` pentru a obține elementul la index
    Color::parse(hex).unwrap_or(Color::BLACK) // Folosim negru daca parsing-ul eșueaza
}
pub fn button_style(theme: &Theme, status: Status,tema: &Tema,culoare: Color) -> Style {
    match theme
    {
        Theme::Light
        => {
            match status {
                Status::Active => Style {
                    background: Some(Background::Color(culori(1, tema))), // alb
                    text_color: culori(0, tema),                         // negru
                    border: Border {
                        color: culori(0, tema), // negru
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
                    background: Some(Background::Color(culori(2, tema))), // alb
                    text_color: culori(0, tema),                         // negru
                    border: Border {
                        color: culori(1, tema), // negru
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
                    background: Some(Background::Color(culori(1, tema))), // alb
                    text_color: culori(0, tema),                         // negru
                    border: Border {
                        color: culori(0, tema), // negru
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
                    background: Some(Background::Color(culori(1, tema))), // alb
                    text_color: culori(5, tema),                         // negru
                    border: Border {
                        color: culori(0, tema), // negru
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
        },

        Theme::Dracula =>
            {
                Style {
                    background: Some(Background::Color(culori(2, tema))), // alb
                    text_color: culori(0, tema),                         // negru
                    border: Border {
                        color: culori(1, tema), // negru
                        width: 2.0,
                        radius: Radius::new(5.0),
                    },
                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 4.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                    },

                }
            },
        Theme::Ferra =>
            {
                match status {
                    Status::Active => Style {
                        background: Some(Background::Color(Color::WHITE)), // alb
                        text_color:Color::parse("11100f").unwrap(),                         // negru
                        border: Border {
                            color: Color::parse("11100f").unwrap(), // negru
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
                        background: Some(Background::Color(culori(2, tema))), // alb
                        text_color: culori(0, tema),                         // negru
                        border: Border {
                            color: culori(1, tema), // negru
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
                        background: Some(Background::Color(culori(1, tema))), // alb
                        text_color: culori(0, tema),                         // negru
                        border: Border {
                            color: culori(0, tema), // negru
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
                        background: Some(Background::Color(culori(1, tema))), // alb
                        text_color: culori(5, tema),                         // negru
                        border: Border {
                            color: culori(0, tema), // negru
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


    },
        _ =>
            {
                Style {
                    background: Some(Background::Color(culoare)), // alb
                    text_color: culori(0, tema),                         // negru
                    border: Border {
                        color: (culoare), // negru
                        width: 5.0,
                        radius: Radius::new(50.0),
                    },
                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 0.0,
                        color: Color::TRANSPARENT,
                    }
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
                background: Some(Background::Color(culori(1, tema))), // alb
                text_color: Some(culori(0, tema)),                         // negru
                border: Border {
                    color: culori(0, tema), // negru
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
                background: Some(Background::Color(culori(2, tema))), // alb
                text_color: Some(culori(0, tema)),                         // negru
                border: Border {
                    color: culori(1, tema), // negru
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
                background: Some(Background::Color(culori(1, tema))), // alb
                text_color: Some(culori(0, tema)),                         // negru
                border: Border {
                    color: culori(0, tema), // negru
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
                    background: Some(Background::Color(culoare)), // alb
                    text_color: Some(culori(0, tema)),                         // negru
                    border: Border {
                        color: (culoare), // negru
                        width: 5.0,
                        radius: Radius::new(50.0),
                    },
                    shadow: Shadow {
                        offset: iced::Vector::new(0.0, 0.0),
                        blur_radius: 0.0,
                        color: Color::TRANSPARENT,
                    }
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
                background: Background::Color(culori(1, tema)), // alb
                                   // negru
                border: Border {
                    color: culori(0, tema), // negru
                    width: 2.0,
                    radius: Radius::new(5.0),
                },
                 icon: Color::TRANSPARENT,
                 placeholder: (culori(0, tema)),
                 value: (culori(0, tema)),
                 selection: (culori(2, tema)),
            },
            text_input::Status::Focused {is_hovered:_} => text_input::Style {
                background: Background::Color(culori(2, tema)), // alb
                                        // negru
                border: Border {
                    color: culori(1, tema), // negru
                    width: 2.0,
                    radius: Radius::new(5.0),
                },
                icon: Color::TRANSPARENT,
                placeholder: (culori(0, tema)),
                value: (culori(0, tema)),
                selection: (culori(3, tema)),
            },
            _ => text_input::Style {
                background: (Background::Color(culori(1, tema))), // alb negru
                border: Border {
                    color: culori(0, tema), // negru
                    width: 2.0,
                    radius: Radius::new(5.0),
                },
                icon: Color::TRANSPARENT,
                placeholder: (culori(0, tema)),
                value: (culori(0, tema)),
                selection: (culori(0, tema)),
            },


        }

        _ =>
            {
                text_input::Style {
                    background: (Background::Color(culoare)), // alb
                    // negru
                    border: Border {
                        color: (culoare), // negru
                        width: 5.0,
                        radius: Radius::new(50.0),
                    },
                    icon: Color::TRANSPARENT,
                    placeholder: (culori(0, tema)),
                    value: (culori(0, tema)),
                    selection: (culori(0, tema)),
                }
            }
    }
}
