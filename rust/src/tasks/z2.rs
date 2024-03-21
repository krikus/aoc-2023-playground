use std::{cmp, str::FromStr};

struct GameTurn {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    games: Vec<GameTurn>,
}

fn convert(s: &String) -> Game {
    let game = s.split_once(":").and_then(|(x, y)| {
        let num = x.rsplit_once(" ").and_then(|num| num.1.into()).expect("!");

        let games = y.split(";").map(|s| {
            s.split(",").map(|f| f.trim().split_once(" ")).fold(
                GameTurn {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, cur| match cur {
                    Some((val, "green")) => {
                        acc.green = FromStr::from_str(val).unwrap();
                        acc
                    }
                    Some((val, "blue")) => {
                        acc.blue = FromStr::from_str(val).unwrap();
                        acc
                    }
                    Some((val, "red")) => {
                        acc.red = FromStr::from_str(val).unwrap();
                        acc
                    }
                    _ => acc,
                },
            )
        });

        let answer = Game {
            id: FromStr::from_str(num).unwrap(),
            games: games.collect(),
        };

        return Some(answer);
    });

    return game.expect("Something went wrong");
}

pub fn solve(lines: Vec<String>) {
    let games: Vec<Game> = lines.iter().map(convert).collect();

    let max_blue = 14;
    let max_red = 12;
    let max_green = 13;
    let ans1: i32 = games
        .iter()
        .filter(|game| {
            game.games
                .iter()
                .all(|g| g.blue <= max_blue && max_red >= g.red && max_green >= g.green)
        })
        .map(|g| g.id)
        .sum();

    print!("Result #1: {}\n", ans1);

    let ans2: i32 = games
        .iter()
        .map(|game| {
            let gt = game.games.iter().fold(
                GameTurn {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut a, c| {
                    a.red = cmp::max(a.red, c.red);
                    a.blue = cmp::max(a.blue, c.blue);
                    a.green = cmp::max(a.green, c.green);
                    a
                },
            );

            gt.green * gt.blue * gt.red
        })
        .sum();

    print!("Result #2: {}\n", ans2)
}
