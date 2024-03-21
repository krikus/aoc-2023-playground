use regex::Regex;
use std::collections::HashMap;

struct GraphNode {
    name: String,
    left_name: String,
    right_name: String,
}

struct GraphNodePointer<'a> {
    node: &'a GraphNode,
}

struct GraphNodePointers<'a> {
    nodes: Vec<&'a GraphNode>,
}

pub fn solve(lines: Vec<String>) {
    let path = &lines[0];
    let ex: Regex = Regex::new("[A-Z0-9]{3}").unwrap();
    let nodes: Vec<GraphNode> = lines
        .iter()
        .skip(2)
        .map(|line| {
            let x: Vec<&str> = ex.find_iter(line).map(|r| r.as_str()).collect();
            GraphNode {
                name: x[0].to_string(),
                left_name: x[1].to_string(),
                right_name: x[2].to_string(),
            }
        })
        .collect();

    let node_map = nodes.iter().fold(
        HashMap::new(),
        |mut hmap: HashMap<&str, &GraphNode>, node| {
            hmap.insert(&node.name, node);
            hmap
        },
    );

    let ans: i32 = path
        .chars()
        .cycle()
        .zip(2..)
        .scan(
            GraphNodePointer {
                node: node_map.get("AAA").unwrap(),
            },
            |state, (direction, step)| {
                let current = &state.node;
                let new_node_name = match direction {
                    'L' => &current.left_name,
                    'R' => &current.right_name,
                    _ => {
                        panic!("Bad char in the input cycle")
                    }
                };
                let node = node_map.get(&new_node_name as &str).unwrap();

                state.node = node;

                Some((node, step))
            },
        )
        .take(2)
        .take_while(|(node, _)| node.name != "ZZZ")
        .last()
        .map(|(_, x)| x)
        .unwrap();

    println!("Result#1: {}\n", ans);

    let all_start_nodes = node_map.values().filter(|x| x.name.ends_with("A"));
    println!("X {}", all_start_nodes.clone().count());

    let ans2: i32 = path
        .chars()
        .cycle()
        .scan(
            GraphNodePointers {
                nodes: all_start_nodes.map(|x| *x).collect(),
            },
            |states, direction| {
                states.nodes = states
                    .nodes
                    .iter()
                    .map(|current| {
                        let new_node_name = match direction {
                            'L' => &current.left_name,
                            'R' => &current.right_name,
                            _ => {
                                panic!("Bad char in the input cycle")
                            }
                        };
                        let node = *node_map.get(&new_node_name as &str).unwrap();

                        node
                    })
                    .collect();
                Some(states.nodes.clone())
            },
        )
        .take_while(|nodes| !nodes.iter().all(|n| n.name.ends_with("Z")))
        .zip(2..)
        .last()
        .map(|(_, x)| x)
        .unwrap();

    println!("Result#1: {}\n", ans);
    println!("Result#2: {}\n", ans2);
}
