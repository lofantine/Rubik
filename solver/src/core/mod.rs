mod color_set;
use colored::Colorize;

use std::{print, println, todo};
use std::collections::VecDeque;


use color_set::ColorSet;



pub trait Printable {
    fn print(&self);
}

#[derive(Debug, Clone)]
struct Face(Vec<Vec<ColorSet>>);

impl Face {
    fn new(_size: usize, _color: Option<ColorSet>) -> Self {
        Self(vec![vec![_color.unwrap_or(ColorSet::WHITE); _size]; _size])
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
            Face::new(_size, Some(ColorSet::WHITE)),
            Face::new(_size, Some(ColorSet::GREEN)),
            Face::new(_size, Some(ColorSet::RED)),
            Face::new(_size, Some(ColorSet::BLUE)),
            Face::new(_size, Some(ColorSet::ORANGE)),
            Face::new(_size, Some(ColorSet::YELLOW)),
        ])
    }

    pub fn mov(&mut self, movement: AbsoluteMovement) {
        match movement {
            AbsoluteMovement::Top => self.move_top(),
            _ => ()
        }
    }

    fn move_top(&mut self) {
        // move face itself
        let mut queue : VecDeque<ColorSet> = VecDeque::new();
        
    }

    fn move_top_r(&mut self) {
        
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
