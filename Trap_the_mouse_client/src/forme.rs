use crate::messages::Message;
use iced::{
    widget::canvas::{self, Canvas, Frame, Geometry, Path},
     Element, Length, Point, Rectangle, Renderer, Size,Task
};
use iced::{  Theme,Border,Shadow};
use iced::Color;
use iced::Background;
use iced::widget::button::Status::{Active,Hovered as Hover,Disabled,Pressed};
use iced::widget::button::Style;
use iced::border::Radius;
use std::sync::Arc;
use iced::widget::button::Status;
use iced::widget::{button, container, Button};
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
        2 =>set_tema.COLORS= ["f6e8df".to_string(),"3e3c6e".to_string(), "fe979b".to_string(), "feae97".to_string()],
        //AQUA
        3 =>set_tema.COLORS= ["50adbf".to_string(),"193841".to_string(), "5d1c34".to_string(),"2171b5".to_string()],
        //DEFAULT
        4 =>set_tema.COLORS= ["1b1b1b".to_string(),"77abb6".to_string(), "f1ac20".to_string(),"023336".to_string()],
        //Sniper
        _ =>set_tema.COLORS= ["5d1c34".to_string(),"48a0a3".to_string(), "9cbbfc".to_string(),"d9eff7".to_string()],
    }
}

pub fn culori(index: usize, tema: &Tema) -> Color {
    let fallback = "ffffff".to_string(); // Alb ca fallback
    let hex = tema.COLORS.get(index).unwrap_or(&fallback); // Folosim `.get` pentru a obține elementul la index
    Color::parse(hex).unwrap_or(Color::BLACK) // Folosim negru daca parsing-ul eșueaza
}
pub fn button_style(theme: &Theme, status: Status,tema: &Tema) -> Style {

      match status {
            Status::Active => Style {
                background: Some(Background::Color(culori(1,tema))), // alb
                text_color: culori(0,tema),                         // negru
                border: Border {
                    color: culori(0,tema), // negru
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
                background: Some(Background::Color(culori(2,tema))), // alb
                text_color: culori(0,tema),                         // negru
                border: Border {
                    color: culori(1,tema), // negru
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
                background: Some(Background::Color(culori(1,tema))), // alb
                text_color: culori(0,tema),                         // negru
                border: Border {
                    color: culori(0,tema), // negru
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
                background: Some(Background::Color(culori(1,tema))), // alb
                text_color: culori(5,tema),                         // negru
                border: Border {
                    color: culori(0,tema), // negru
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
