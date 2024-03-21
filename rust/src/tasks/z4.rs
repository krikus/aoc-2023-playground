use std::collections::HashSet;

struct Card {
    id: i32,
    numbers: HashSet<i32>,
    winning: HashSet<i32>,
}

fn str_to_card(s: &String) -> Card {
    let x: Vec<HashSet<i32>> = s
        .split_terminator(&[':', '|'][..])
        .map(|part| {
            part.split(" ")
                .map(|s| s.parse())
                .filter(|x: &Result<i32, _>| x.is_ok())
        })
        .map(|f| f.map(|num| num.unwrap()))
        .map(|x| x.collect())
        .collect();

    let my_struct: Card = Card {
        id: *x[0].iter().last().unwrap(),
        numbers: x[1].to_owned(),
        winning: x[2].to_owned(),
    };
    return my_struct;
}

pub fn solve<'a>(lines: Vec<String>) {
    let cards: Vec<Card> = lines.iter().map(str_to_card).collect();

    let ans = cards
        .iter()
        .map(|x| x.numbers.intersection(&x.winning))
        .map(|x| x.count())
        .map(|x| match x {
            0 => 0,
            e => i32::pow(2, (e - 1).try_into().unwrap()),
        })
        .reduce(|x, y| x + y)
        .unwrap();

    println!("Result#1 {}\n", ans);
    println!("Result#2 TODO\n");
}
