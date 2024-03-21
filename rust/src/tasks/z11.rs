use crate::utils::geometry::Point as PointT;
use std::{char, fmt::Write};

type Point = PointT<i128>;

struct Map2d {
    mp: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

struct Map2dRowIterator<'a> {
    row: usize,
    cur: i32,
    map2d: &'a Map2d,
}

impl<'a> Iterator for Map2dRowIterator<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        self.map2d.get_at(self.cur, self.row as i32)
    }
}

struct Map2dColumnIterator<'a> {
    cur: i32,
    column: usize,
    map2d: &'a Map2d,
}

impl<'a> Iterator for Map2dColumnIterator<'a> {
    type Item = &'a char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        self.map2d.get_at(self.column as i32, self.cur)
    }
}

impl<'b> Map2d {
    fn from<'a, T>(x: T) -> Map2d
    where
        T: Iterator<Item = &'a str>,
    {
        let chars = x.map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
        let height = chars.len();
        let width = chars[0].len();

        return Map2d {
            mp: chars,
            width,
            height,
        };
    }

    fn get_at(&self, x: i32, y: i32) -> Option<&char> {
        let point = match self.mp.get(y as usize) {
            Some(row) => row.get(x as usize).map(|x| x),
            _ => None,
        };

        point
    }

    fn find_positions(&self, of: char) -> Box<[Point]> {
        let mut points: Vec<Point> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let px = x as i32;
                let py = y as i32;
                match self.get_at(px, py) {
                    Some('#') => points.push(Point {
                        x: px as i128,
                        y: py as i128,
                    }),
                    _ => {}
                }
            }
        }
        points.into_boxed_slice()
    }

    fn iter_row(&self, row: usize) -> Map2dRowIterator {
        return Map2dRowIterator {
            cur: -1,
            row: row,
            map2d: self,
        };
    }

    fn iter_column(&self, column: usize) -> Map2dColumnIterator {
        return Map2dColumnIterator {
            cur: -1,
            column: column,
            map2d: self,
        };
    }
}

impl std::fmt::Display for Map2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str("\n====map2d====\n");
        self.mp.iter().for_each(|x| {
            x.iter().for_each(|y| {
                let _ = f.write_char(*y);
            });
            let _ = f.write_str("\n");
        });
        let _ = f.write_str("\n=============");
        Ok(())
    }
}

pub fn solve(lines: &Vec<String>) {
    let map2d = Map2d::from(lines.iter().map(|f| f.as_ref()));
    map2d.get_at(0, 2);
    let expand_by = 1000000 - 1;
    let mut points = map2d.find_positions('#');
    let mut expand_rows: Vec<usize> = vec![];
    let mut expand_columns: Vec<usize> = vec![];

    for i in 0..map2d.width {
        if map2d.iter_column(i).all(|x| *x == '.') {
            expand_columns.push(i);
        }
    }

    for i in 0..map2d.height {
        if map2d.iter_row(i).all(|x| *x == '.') {
            expand_rows.push(i);
        }
    }

    println!(
        "rows: {}",
        &expand_rows
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("-")
    );

    println!(
        "columns: {}",
        &expand_columns
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("-")
    );

    for p in points.iter_mut() {
        let mut ex = 0;
        let mut ey = 0;
        for row in expand_rows.iter() {
            if p.y as usize > *row {
                ey += expand_by
            }
        }
        for column in expand_columns.iter() {
            if p.x as usize > *column {
                ex += expand_by
            }
        }
        if ex != 0 || ey != 0 {
            println!("Increasing {} by x= {} y= {}", p, ex, ey);
        }
        p.x += ex;
        p.y += ey;
    }

    points.iter().for_each(|p| println!("Point# {}", p));

    let ans: i128 = points
        .iter()
        .enumerate()
        .flat_map(|(n, p1)| points.iter().skip(n + 1).map(move |p2| (p1, p2)))
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .sum();
    println!("Result#1: {}", ans)
}
