use std::fmt::Write;

struct MagicNumbers {
    nums: MagicRow,
    down: Option<Vec<MagicRow>>,
}

type MagicRow = Vec<i32>;

impl MagicNumbers {
    fn calc_last(&mut self) -> i32 {
        self.calc_down()
            .iter()
            .rev()
            .chain(std::iter::once(&self.nums))
            .fold(0, |state, row| row.last().unwrap_or(&0) + state)
    }
    fn calc_first(&mut self) -> i32 {
        self.calc_down()
            .iter()
            .rev()
            .chain(std::iter::once(&self.nums))
            .fold(0, |state, row| row.first().unwrap_or(&0) - state)
    }
    fn calc_down(&mut self) -> Vec<MagicRow> {
        struct tmp {
            last: MagicRow,
        }

        self.down
            .get_or_insert_with(|| {
                let all: Vec<MagicRow> = (0..)
                    .scan(
                        tmp {
                            last: self.nums.clone(),
                        },
                        |row, _| {
                            let new_row = row
                                .last
                                .windows(2)
                                .map(|a| a[1] - a[0])
                                .collect::<MagicRow>();

                            row.last = new_row;
                            return Some(row.last.to_owned());
                        },
                    )
                    .take_while(|row| !row.iter().all(|x| *x == 0))
                    .collect();
                all
            })
            .to_vec()
    }
}

impl std::fmt::Display for MagicNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .nums
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        let _ = f.write_char('{');
        let _ = f.write_str(&str);
        let _ = f.write_char('}');
        Result::Ok(())
    }
}

pub fn solve(lines: Vec<String>) {
    let numbers_iter = lines
        .iter()
        .map(|f| f.split(" ").map(|f| f.parse::<i32>().unwrap()))
        .map(|x| MagicNumbers {
            nums: x.collect(),
            down: None,
        });

    let (ans1, ans2) = numbers_iter
        .map(|mut x| (x.calc_last(), x.calc_first()))
        .reduce(|(s1, s2), (n1, n2)| (s1 + n1, s2 + n2))
        .unwrap();

    println!("Result#1 {}\n", ans1);
    println!("Result#2 {}\n", ans2);
}
