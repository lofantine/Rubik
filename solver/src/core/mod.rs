mod color_set;
use colored::Colorize;

use std::collections::VecDeque;
use std::{print, println, todo};

use color_set::ColorSet;

#[derive(Debug, Clone)]
#[repr(usize)]
pub enum IntFace {
    Top = 0,
    Bottom = 1,
    Front = 2,
    Back = 3,
    Left = 4,
    Right = 5,
}

pub trait Printable {
    fn print(&self);
}

#[derive(Debug, Clone)]
struct Face(Vec<Vec<ColorSet>>);

impl Face {
    fn new(_size: usize, _color: Option<ColorSet>) -> Self {
        Self(vec![vec![_color.unwrap_or(ColorSet::WHITE); _size]; _size])
    }

    fn rotate(&mut self, o_step: Option<i32>) -> () {
        let step = ((o_step.unwrap_or(1) % 4) + 4) % 4; // forcer le positif
        println!("Step to rotate : {}", step);
        if step == 0 {
            return;
        }
        let length = self.0.len();

        for row in 0..length / 2 {
            for col in row..=length / 2 {
                let mut elements: [(usize, usize); 4] = [(col, row), (0, 0), (0, 0), (0, 0)];

                for i in 0..3 {
                    // exec 4 * pour bien passer tous les cote
                    elements[i + 1].0 = length - (elements[i].1 + 1);
                    elements[i + 1].1 = elements[i].0;
                }

                let mut elements_color: [ColorSet; 4] = [
                    self.0
                        .get(elements[0].1)
                        .unwrap()
                        .get(elements[0].0)
                        .unwrap()
                        .clone(),
                    self.0
                        .get(elements[1].1)
                        .unwrap()
                        .get(elements[1].0)
                        .unwrap()
                        .clone(),
                    self.0
                        .get(elements[2].1)
                        .unwrap()
                        .get(elements[2].0)
                        .unwrap()
                        .clone(),
                    self.0
                        .get(elements[3].1)
                        .unwrap()
                        .get(elements[3].0)
                        .unwrap()
                        .clone(),
                ];

                match step {
                    3 => elements_color.rotate_left(1),
                    x => elements_color.rotate_right(x as usize),
                }

                for (i, e) in elements.iter_mut().enumerate() {
                    *self.0.get_mut(e.1).unwrap().get_mut(e.0).unwrap() = elements_color[i];
                }
            }
        }
    }
}

impl Printable for Face {
    fn print(&self) {
        println!("F [ ");
        for row in self.0.iter() {
            for col in row.iter() {
                print!("{}", col);
            }
            print!("\n");
        }
        println!("]");
    }
}

pub enum AbsoluteMovement {
    Top,
    Front,
    Left,
    Right,
    Back,
    Bottom,
}

pub enum Movement {
    Top,
    Middle,
    Bottom,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Cube(Vec<Face>);

impl Cube {
    pub fn new(_size: usize) -> Self {
        Self(vec![
            Face::new(_size, Some(ColorSet::WHITE)),  // top
            Face::new(_size, Some(ColorSet::GREEN)),  // bottom
            Face::new(_size, Some(ColorSet::RED)),    // front
            Face::new(_size, Some(ColorSet::BLUE)),   // back
            Face::new(_size, Some(ColorSet::ORANGE)), // left
            Face::new(_size, Some(ColorSet::YELLOW)), // right
        ])
    }

    pub fn mov(&mut self, movement: AbsoluteMovement) {
        match movement {
            AbsoluteMovement::Top => self.move_top(),
            _ => (),
        }
    }

    fn move_top(&mut self) {
        let top_face = self.0.get_mut(IntFace::Top as usize).unwrap();
        top_face.rotate(Some(1));
    }

    fn move_top_r(&mut self) {
        todo!("doing in reverse order");
    }
}

impl Printable for Cube {
    fn print(&self) {
        println!("{}", "------- starting printing cube --------".cyan());
        println!(
            "Size of the cube : {}",
            self.0.get(0).unwrap().0.len().to_string().as_str().green()
        );
        for face in &self.0 {
            face.print();
        }
        println!("{}", "------- cube printed --------".cyan());
    }
}

#[test]
fn test_faces() {
    let mut face = Face::new(3, Some(ColorSet::WHITE));
    *face.0.get_mut(0).unwrap().get_mut(1).unwrap() = ColorSet::ORANGE;
    *face.0.get_mut(0).unwrap().get_mut(0).unwrap() = ColorSet::BLUE;
    *face.0.get_mut(1).unwrap().get_mut(1).unwrap() = ColorSet::GREEN;
    face.print();
    face.rotate(Some(1));
    face.print();
    face.rotate(Some(1));
    face.print();
    face.rotate(Some(1));
    face.print();
    face.rotate(Some(-1));
    face.print();
    let mut face2 = Face::new(5, Some(ColorSet::WHITE));
    *face2.0.get_mut(0).unwrap().get_mut(1).unwrap() = ColorSet::ORANGE;
    *face2.0.get_mut(0).unwrap().get_mut(0).unwrap() = ColorSet::BLUE;
    *face2.0.get_mut(0).unwrap().get_mut(2).unwrap() = ColorSet::GREEN;
    *face2.0.get_mut(1).unwrap().get_mut(2).unwrap() = ColorSet::GREEN;
    face2.print();
    face2.rotate(Some(1));
    face2.print();
    face2.rotate(Some(1));
    face2.print();
    face2.rotate(Some(1));
    face2.print();
    face2.rotate(Some(-1));
    face2.print();
}
