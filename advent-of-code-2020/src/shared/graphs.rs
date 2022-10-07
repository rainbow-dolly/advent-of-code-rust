use std::{collections::{HashMap, HashSet}, hash::Hash};

pub struct Graph<T: Eq + Hash + Clone> {
    nodes: HashSet<T>,
    edges: HashMap<T, Vec<(i64, T)>>
}

impl <T: Eq + Hash + Clone> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new()
        }
    }

    pub fn get_nodes(&self) -> Vec<T> {
        Vec::from_iter(self.nodes.clone())
    }

    pub fn get_edges_from(&self, source: &T) -> Vec<(i64, T)> {
        self.edges.get(source).unwrap().clone()
    }

    pub fn add_edge(&mut self, source: &T, destination: &T, weight: i64) {
        if self.nodes.insert(source.clone()) {
            self.edges.insert(source.clone(), vec![(weight, destination.clone())]);
        } else {
            self.edges.get_mut(source).unwrap().push((weight, destination.clone()));
        }

        if self.nodes.insert(destination.clone()) {
            self.edges.insert(destination.clone(), vec![]);
        }
    }

    pub fn path_exists(&self, source: &T, destination: &T) -> bool {
        let mut visited: HashSet<&T> = HashSet::new();
        let mut queue: Vec<&T> = Vec::new();
        let mut current_node: &T;

        queue.push(source);

        while !queue.is_empty() {
            current_node = queue.pop().unwrap();
            visited.insert(current_node);

            for (_, candidate) in self.edges.get(current_node).unwrap() {
                if candidate == destination {
                    return true;
                }
                
                if !visited.contains(candidate) {
                    queue.push(candidate);
                }
            }
        }

        false
    }
}