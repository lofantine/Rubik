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

    fn rotate(&mut self, o_step: Option<i32>) -> Result<usize, String> {
        let step = ((o_step.unwrap_or(1) % 4) + 4) % 4; // forcer le positif
        if step == 0 {
            return Err("step is 0 you stoobid !".into());
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
        Ok(length)
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
            Face::new(_size, Some(ColorSet::YELLOW)), // bottom
            Face::new(_size, Some(ColorSet::RED)),    // front
            Face::new(_size, Some(ColorSet::ORANGE)), // back
            Face::new(_size, Some(ColorSet::GREEN)),  // left
            Face::new(_size, Some(ColorSet::BLUE)),   // right
        ])
    }

    fn get_two_face_mut(&mut self, face1: IntFace, face2: IntFace) -> (&mut Face, &mut Face) {
        let (i1, i2): (usize, usize) = (face1 as usize, face2 as usize);

        assert_ne!(i1, i2);

        if i1 < i2 {
            let (v1, v2) = self.0.split_at_mut(i2);
            (&mut v1[i1], v2.first_mut().unwrap())
        } else {
            let (v1, v2) = self.0.split_at_mut(i1);
            (v2.first_mut().unwrap(), &mut v1[i2])
        }
    }

        
    pub fn mov(&mut self, movement: AbsoluteMovement, step: i32) {
        match movement {
            AbsoluteMovement::Top => self.move_top(Some(step)),
            AbsoluteMovement::Bottom => self.move_bottom(Some(step)),
            AbsoluteMovement::Left => self.move_left(Some(step)),
            AbsoluteMovement::Right => self.move_right(Some(step)),
            AbsoluteMovement::Front => self.move_front(Some(step)),
            AbsoluteMovement::Back => self.move_back(Some(step)),
            _ => (),
        }
    }

    fn move_top(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj { 'T' } else { 't' });
        }
        if step == 0 {
            return ;
        }

        // moving top_face itslf
        let top_face = self.0.get_mut(IntFace::Top as usize).unwrap();
        let _length = top_face
            .rotate(Some(step))
            .expect("Not supposed to error here -  mov_top()");

        // moving the side faces
        // front -> left -> back -> right -> front

        let mut order = [IntFace::Left, IntFace::Back, IntFace::Right];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        for _ in 0..step {
            
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Front, order[i].clone());
                match order[i] {
                    IntFace::Back => {
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                        );
                    }
                    _ => std::mem::swap(f1.0.get_mut(0).unwrap(), f2.0.get_mut(0).unwrap()),
                }
                // drop(f1, f2)
            }
        }
        /* Les drop ne sont pas obligatoire pour 2 raisons :
         * - Ce sont des referene et pas des values eux meme ( consomme justele reference )
         * - !! Comme les reference ne sont pas reutilises apres le swap, Rust authorize un autre empreint as mut ( car pas de changement en + )
         */
    }

    fn move_bottom(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj {'B'} else {'b'});
        }

        if step == 0 {
            return ;
        }
        
        let bottom_face = self.0.get_mut(IntFace::Bottom as usize).unwrap();
        let _length = bottom_face
            .rotate(Some(step))
            .expect("Not supprose to Error here - move_bottom()");

        let mut order = [IntFace::Right, IntFace::Back, IntFace::Left];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        // now moving side face
        for _ in 0..step {
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Front, order[i].clone());
                match order[i] {
                    IntFace::Back => {
                        std::mem::swap(
                            f1.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                            f2.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                            f2.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                            f2.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                        );
                    }
                    _ => std::mem::swap(f1.0.get_mut(2).unwrap(), f2.0.get_mut(2).unwrap()),
                }
            }
        }
    }

    fn move_left(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj {'L'} else {'l'});
        }

        if step == 0 {
             return ;
        }
        
        let left_face = self.0.get_mut(IntFace::Left as usize).unwrap();
        let _length = left_face
            .rotate(Some(step))
            .expect("Not supprose to Error here - move_bottom()");

        // based on bottom face pivot
        let mut order = [IntFace::Back, IntFace::Top, IntFace::Front];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        for _ in 0..step {
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Bottom, order[i].clone());
                std::mem::swap(
                    f1.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                    f2.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                );
                std::mem::swap(
                    f1.0.get_mut(1).unwrap().get_mut(0).unwrap(),
                    f2.0.get_mut(1).unwrap().get_mut(0).unwrap(),
                );
                std::mem::swap(
                    f1.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                    f2.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                );
            }
        }
    }

    fn move_right(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj {'R'} else {'r'});
        }

        if step == 0 {
            return ;
        }
        
        let right_face = self.0.get_mut(IntFace::Right as usize).unwrap();
        let _length = right_face
            .rotate(Some(step))
            .expect("Not supprose to Error here - move_bottom()");

        // based on bottom face pivot
        let mut order = [IntFace::Front, IntFace::Top, IntFace::Back];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        for _ in 0..step {
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Bottom, order[i].clone());
                std::mem::swap(
                    f1.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                    f2.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                );
                std::mem::swap(
                    f1.0.get_mut(1).unwrap().get_mut(2).unwrap(),
                    f2.0.get_mut(1).unwrap().get_mut(2).unwrap(),
                );
                std::mem::swap(
                    f1.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                    f2.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                );
            }
        }
    }

    fn move_front(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj {'F'} else {'f'});
        }

        if step == 0 {
            return ;
        }
        
        let front_face = self.0.get_mut(IntFace::Front as usize).unwrap();
        let _length = front_face
            .rotate(Some(step))
            .expect("Not supprose to Error here - move_bottom()");

        // based on bottom face pivot
        let mut order = [IntFace::Left, IntFace::Top, IntFace::Right];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        for _ in 0..step {
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Bottom, order[i].clone());
                match order[i] {
                    IntFace::Left => {
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            f2.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                            f2.0.get_mut(1).unwrap().get_mut(2).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                        );
                    }
                    IntFace::Top => {
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                        );
                    }
                    IntFace::Right => {
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            f2.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                            f2.0.get_mut(1).unwrap().get_mut(0).unwrap(),
                        );
                        std::mem::swap(
                            f1.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            f2.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                        );
                    }
                    _ => panic!("Not supposed to reach here")
                }
            }
        }
    }

    fn move_back(&mut self, _step: Option<i32>) {
        let mut _step = _step.unwrap_or(1);
        let mut step = ((_step % 4) + 4) % 4;
        let maj = _step >= 0;
        _step = _step.abs();
        for _ in 0.._step {
            print!("{}", if maj {'B'} else {'b'});
        }

        if step == 0 {
            return ;
        }
        
        let back_face = self.0.get_mut(IntFace::Back as usize).unwrap();
        let _length = back_face
            .rotate(Some(step))
            .expect("Not supprose to Error here - move_bottom()");

        // based on bottom face pivot
        let mut order = [IntFace::Right, IntFace::Top, IntFace::Left];
        if step == 3 {
            order.reverse();
            step = 1;
        }

        for _ in 0..step {
            for i in 0..order.len() {
                let (f1, f2) = self.get_two_face_mut(IntFace::Bottom, order[i].clone());
                match order[i] {
                    IntFace::Left => {
                        std::mem::swap(
                            f1.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                                f2.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                                f2.0.get_mut(1).unwrap().get_mut(0).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                                f2.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                            );
                        }
                        IntFace::Top => {
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                                f2.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                                f2.0.get_mut(0).unwrap().get_mut(1).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                                f2.0.get_mut(0).unwrap().get_mut(0).unwrap(),
                            );
                        }
                        IntFace::Right => {
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(0).unwrap(),
                                f2.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(1).unwrap(),
                                f2.0.get_mut(1).unwrap().get_mut(2).unwrap(),
                            );
                            std::mem::swap(
                                f1.0.get_mut(2).unwrap().get_mut(2).unwrap(),
                                f2.0.get_mut(0).unwrap().get_mut(2).unwrap(),
                            );
                        }
                        _ => panic!("Not supposed to reach here")
                    }
                }
            }
        }
}

impl Printable for Cube {
    fn print(&self) {
        println!("\n{}", "------- starting printing cube --------".cyan());
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
    let mut r = face.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face.print();
    r = face.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face.print();
    r = face.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face.print();
    r = face.rotate(Some(-1));
    assert_ne!(r.is_err(), true);
    face.print();
    let mut face2 = Face::new(5, Some(ColorSet::WHITE));
    *face2.0.get_mut(0).unwrap().get_mut(1).unwrap() = ColorSet::ORANGE;
    *face2.0.get_mut(0).unwrap().get_mut(0).unwrap() = ColorSet::BLUE;
    *face2.0.get_mut(0).unwrap().get_mut(2).unwrap() = ColorSet::GREEN;
    *face2.0.get_mut(1).unwrap().get_mut(2).unwrap() = ColorSet::GREEN;
    face2.print();
    r = face2.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face2.print();
    r = face2.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face2.print();
    r = face2.rotate(Some(1));
    assert_ne!(r.is_err(), true);
    face2.print();
    r = face2.rotate(Some(-1));
    assert_ne!(r.is_err(), true);
    face2.print();
}

#[test]
fn test_full_rotation() {
    let mut cube = Cube::new(3);
    cube.print();
    cube.mov(AbsoluteMovement::Bottom, -1);
    cube.mov(AbsoluteMovement::Top, 2);
    cube.mov(AbsoluteMovement::Front, 1);
    cube.mov(AbsoluteMovement::Left, -1);
    cube.mov(AbsoluteMovement::Right, 1);
    cube.mov(AbsoluteMovement::Front, -1);
    cube.mov(AbsoluteMovement::Back, -1);
    cube.print();
}

#[test]
fn test_get_two_face() {
    let mut cube = Cube::new(3);
    let (f1, f2) = cube.get_two_face_mut(IntFace::Front, IntFace::Back);
    f1.print();
    f2.print();
    let (f1, f2) = cube.get_two_face_mut(IntFace::Left, IntFace::Right);
    f1.print();
    f2.print();
    let (f1, f2) = cube.get_two_face_mut(IntFace::Top, IntFace::Bottom);
    f1.print();
    f2.print();
}
