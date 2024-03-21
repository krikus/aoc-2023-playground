pub mod input {
    use std::fs;

    pub fn read_file_lines(path: &str) -> Vec<String> {
        let content: String = fs::read_to_string(path).expect("WTF?");
        let lines: Vec<String> = content.split("\n").map(String::from).collect();
        lines
    }
}

pub mod geometry {
    use std::ops::{Add, Neg, Sub};

    use num::Integer;
    use num::Zero;

    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct Point<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Neg + Zero + Integer,
    {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Neg + Zero + Integer + Copy,
    {
        pub fn cmp(&self, p2: &Point<T>) -> bool {
            self.x == p2.x && self.y == p2.y
        }

        pub fn manhattan_distance(&self, p2: &Point<T>) -> T {
            let x: T = p2.x - self.x;
            let y: T = p2.y - self.y;
            let absx: T = if x < T::zero() { T::zero() - x } else { x };
            let absy: T = if y < T::zero() { T::zero() - y } else { y };
            return absx + absy;
        }
    }

    impl<T> std::fmt::Display for Point<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Neg + Zero + Integer + std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(format!("{{ x: {}, y: {} }}", self.x, self.y).as_str())
        }
    }
}
