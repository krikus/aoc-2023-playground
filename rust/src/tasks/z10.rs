use std::fmt::Write;

use crate::utils::geometry::Point as PointT;

type Point = PointT<i32>;

#[derive(Clone, Debug)]
enum Pipes {
    Start,
    DIR(Box<[Point]>, char),
    None,
    Path,
}

impl std::fmt::Display for Pipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipes::None => '.',
            Pipes::DIR(_, c) => *c,
            Pipes::Start => 'S',
            Pipes::Path => '*',
        };

        f.write_char(c)
    }
}

#[derive(Clone)]
struct Maze {
    map2d: Vec<Vec<Pipes>>,
    start_point: Point,
}

struct MazeIterator<'a> {
    position: Point,
    maze: &'a mut Maze,
    ban: Point,
    visited: bool,
}

impl Iterator for MazeIterator<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.maze.get_at(self.position.x, self.position.y) {
            Some(Pipes::DIR(p, _)) => p
                .iter()
                .map(|p| Point {
                    x: p.x + self.position.x,
                    y: self.position.y + p.y,
                })
                .find(|p| {
                    !p.cmp(&self.ban)
                        && match self.maze.get_at(p.x, p.y) {
                            Some(Pipes::DIR(moves, _)) => moves
                                .iter()
                                .map(|f| Point {
                                    x: p.x + f.x,
                                    y: p.y + f.y,
                                })
                                .find(|p2| p2.cmp(&self.position))
                                .is_some(),
                            _ => false,
                        }
                }),
            _ => None,
        };

        if let Some(next_point) = next {
            println!("Moving {} -> {}", self.position, next_point);
            println!(
                "Which is {} -> {}",
                self.maze.get_at(self.position.x, self.position.y).unwrap(),
                self.maze.get_at(next_point.x, next_point.y).unwrap()
            );
            self.ban = self.position;
            self.position = next_point;
            if self.visited {
                return None;
            }
            self.visited = self.position.cmp(&self.maze.start_point);
        }

        return next;
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map2d.iter().for_each(|row| {
            row.iter().for_each(|p| {
                let _ = f.write_str(format!("{}", p).as_str());
            });
            let _ = f.write_str("\n");
        });
        Result::Ok(())
    }
}

impl Maze {
    fn get_at(&self, x: i32, y: i32) -> Option<Pipes> {
        let point = match self.map2d.get(y as usize) {
            Some(row) => row.get(x as usize).map(|x| x.clone()),
            _ => None,
        };

        point
    }

    fn set_at<'a>(&'a mut self, x: i32, y: i32, tmp: Pipes) {
        let value = self.map2d.get(y as usize).and_then(|f| f.get(x as usize));
        match value {
            Some(_) => {
                println!("Setting {} at {}/{}", tmp, x, y);

                self.map2d[y as usize][x as usize] = tmp;
            }
            None => {
                panic!("Setting something out of bounds!?")
            }
        }
    }

    fn iter(&mut self) -> MazeIterator {
        MazeIterator {
            visited: false,
            ban: Point { x: -1, y: -1 },
            position: Point {
                x: self.start_point.x,
                y: self.start_point.y,
            },
            maze: self,
        }
    }

    fn solve_maze(&mut self) -> Result<Box<[Point]>, ()> {
        let start = self.start_point;
        let tmp = self.iter().enumerate().map(|(_, y)| y);

        let vec = tmp.collect::<Vec<Point>>();

        if let Some((p)) = vec.last() {
            if p.cmp(&start) {
                return Ok(vec.into_boxed_slice());
            }
        }

        Err(())
    }
}

fn new_maze<'a>(lines: &'a Vec<String>) -> Maze {
    // ## PARSE ##
    let mut all_pipes: Vec<Vec<Pipes>> = vec![];
    let mut start_point = Point { x: 0, y: 0 };
    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<Pipes> = line
            .chars()
            .enumerate()
            .map(|(j, c)| match c {
                'S' => {
                    start_point.x = j as i32;
                    start_point.y = i as i32;
                    Pipes::Start
                }
                'J' => Pipes::DIR(
                    Box::new([Point { x: -1, y: 0 }, Point { x: 0, y: -1 }]),
                    'J',
                ),
                '7' => Pipes::DIR(Box::new([Point { x: -1, y: 0 }, Point { x: 0, y: 1 }]), '7'),
                'F' => Pipes::DIR(Box::new([Point { x: 1, y: 0 }, Point { x: 0, y: 1 }]), 'F'),
                'L' => Pipes::DIR(Box::new([Point { x: 1, y: 0 }, Point { x: 0, y: -1 }]), 'L'),
                '|' => Pipes::DIR(Box::new([Point { x: 0, y: 1 }, Point { x: 0, y: -1 }]), '|'),
                '-' => Pipes::DIR(Box::new([Point { x: -1, y: 0 }, Point { x: 1, y: 0 }]), '-'),
                _ => Pipes::None,
            })
            .collect();
        all_pipes.push(chars)
    }
    Maze {
        map2d: all_pipes,
        start_point,
    }
}

pub fn solve<'a>(lines: &Vec<String>) {
    let check: Vec<Pipes> = vec![
        Pipes::DIR(Box::new([Point { x: 1, y: 0 }, Point { x: 0, y: 1 }]), 'F'),
        Pipes::DIR(
            Box::new([Point { x: -1, y: 0 }, Point { x: 0, y: -1 }]),
            'J',
        ),
        Pipes::DIR(Box::new([Point { x: -1, y: 0 }, Point { x: 0, y: 1 }]), '7'),
        Pipes::DIR(Box::new([Point { x: 1, y: 0 }, Point { x: 0, y: -1 }]), 'L'),
        Pipes::DIR(Box::new([Point { x: 0, y: 1 }, Point { x: 0, y: -1 }]), '|'),
        Pipes::DIR(Box::new([Point { x: -1, y: 0 }, Point { x: 1, y: 0 }]), '-'),
    ];

    let wtf = check
        .into_iter()
        .map(|s| {
            let mut mz = new_maze(lines);
            let Point { x, y } = mz.start_point;
            mz.set_at(x, y, s);
            mz.solve_maze()
        })
        .find(|s| s.is_ok())
        .unwrap();

    match wtf {
        Ok(points) => {
            println!("Solution in {}", points.len() / 2);
            let mut mz = new_maze(lines);
            for p in points.iter() {
                mz.set_at(p.x, p.y, Pipes::Path);
            }
            println!("{}", mz);
        }
        _ => println!("No solution"),
    }
    // println!("Result#1: {}", first);
}
