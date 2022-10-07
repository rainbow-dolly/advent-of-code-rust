use itertools::Itertools;

use crate::shared::{self, graphs::Graph};

const MY_BAG: &str = "shiny gold";

pub fn solve_day_7() {
    let inputs = shared::files::fetch_strings_from_file("inputs/day-7.txt");
    let mut graph = Graph::new();
    
    for (source, destinations) in inputs.iter().map(parse_line) {
        for (weight, destination) in destinations {
            graph.add_edge(&source, &destination, weight);
        }
    }

    let part_1 = solve_part_1(&graph);
    let part_2 = solve_part_2(&graph);

    shared::outputs::print_solutions(part_1, part_2);
}

fn parse_line(line: &String) -> (String, Vec<(i64, String)>) {
    let (container, contents) = line.split(" contain ").collect_tuple().unwrap();

    if contents == "no other bags." {
        (parse_bag_without_count(container), Vec::new())
    } else {
        (parse_bag_without_count(container), contents.split(", ").map(parse_bag_with_count).collect_vec())
    }
}

fn parse_bag_without_count(bag: &str) -> String {
    let (color_1, color_2, _) = bag.split(" ").collect_tuple().unwrap();
    format!("{} {}", color_1, color_2)
}

fn parse_bag_with_count(bag: &str) -> (i64, String) {
    let (count, color_1, color_2, _) = bag.split(" ").collect_tuple().unwrap();
    (count.parse().unwrap(), format!("{} {}", color_1, color_2))
}

fn solve_part_1(graph: &Graph<String>) -> i64 {
    let mut paths = 0;
    let puzzle_destination: String = MY_BAG.to_string();

    for node in graph.get_nodes() {
        if graph.path_exists(&node, &puzzle_destination) {
            paths += 1;
        }
    }

    paths
}

fn solve_part_2(graph: &Graph<String>) -> i64 {
    let puzzle_source: String = MY_BAG.to_string();
    
    solve_part_2_rec(graph, &puzzle_source) - 1
}

fn solve_part_2_rec(graph: &Graph<String>, current_node: &String) -> i64 {
    let edges = graph.get_edges_from(current_node);

    if edges.is_empty() {
        1
    } else {
        let mut result = 1;
    
        for (weight, next_node) in edges {
            let value = weight * solve_part_2_rec(graph, &next_node);

            if current_node == "shiny gold" {
                println!("{} {} {}", current_node, next_node, value);
            }
            
            result += value;
        }
    
        result
    }
}