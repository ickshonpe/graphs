extern crate rand;

use rand::seq::SliceRandom;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Graph {
    nodes: Vec<HashSet<usize>>
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        let mut nodes = Vec::with_capacity(n);
        for _ in 0..n {
            nodes.push(HashSet::new());
        }
        Graph {
            nodes
        }
    }

    pub fn get_neighbours(&self, node: usize) -> HashSet<usize> {
        self.nodes[node].clone()
    }

    pub fn add_edge(&mut self, s: usize, t: usize) {
        self.nodes[s].insert(t);
        self.nodes[t].insert(s);
    }

    pub fn remove_edge(&mut self, s: usize, t: usize) {
        self.nodes[s].remove(&t);
        self.nodes[t].remove(&s);
    }

    pub fn remove_edges(&mut self, n: usize) {
        let neighbours = self.get_neighbours(n);
        for neighbour in neighbours {
            self.remove_edge(n, neighbour);
        }    
    }

    pub fn adjecent(&self, s: usize, t: usize) -> bool {
        self.nodes[s].contains(&t)
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    // really slow
    pub fn is_cyclic(&self) -> bool {         
        let mut open = Vec::with_capacity(self.size());            
        let mut g = self.clone();       
        let mut visited = HashSet::new();
        for node in 0..g.size() {            
            if visited.insert(node) {               
                open.push(node);
                while let Some(current) = open.pop() {                
                    let neighbours = g.get_neighbours(current);                                
                    g.remove_edges(current);
                    for &n in &neighbours {                                    
                        if visited.contains(&n) {
                            return true;
                        } else {
                            visited.insert(n);
                            open.push(n);
                        }
                    }
                }
                open.clear();
            } 
        }
        false
    }

    pub fn is_acyclic(&self) -> bool {
        !self.is_cyclic()
    }

    pub fn edges(&self) -> HashSet<(usize, usize)> {
        let mut out = HashSet::new();
        for (node, neighbours) in self.nodes.iter().enumerate() {
            for &adjacent in neighbours {                
                out.insert((min(node, adjacent), max(node, adjacent)));
            }
        } 
        out
    }

    pub fn count_edges(&self) -> usize {
        self.edges().len()
    }
}

fn make_spanning_tree(g: Graph) -> Graph {    
    let mut h = Graph::new(g.size());
    let mut edges: Vec<(usize, usize)> = g.edges().iter().cloned().collect();
    let mut rng = rand::thread_rng();
    edges.shuffle(&mut rng);    
    for (s, t) in edges {
        h.add_edge(s, t);
        if h.is_cyclic() {
            h.remove_edge(s, t);                        
        }
    }
    h
}



#[cfg(test)]
mod tests {
    #[test]
    fn loop_is_cyclic() {
        use Graph;
        let mut g = Graph::new(1);        
        assert!(!g.is_cyclic());
        g.add_edge(0, 0);
        assert!(g.is_cyclic());
    }

    #[test]
    fn three_cycle_is_cyclic() {
        use Graph;
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);        
        assert!(!g.is_cyclic());
        g.add_edge(2, 0);
        assert!(g.is_cyclic());
    }

    #[test]
    fn four_cycle_is_cyclic() {
        use Graph;
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);        
        g.add_edge(2, 3);        
        assert!(!g.is_cyclic());
        g.add_edge(3, 0);
        assert!(g.is_cyclic());
    }

    #[test]
    fn tree_not_cycle() {
        use Graph;
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);        
        g.add_edge(0, 3);        
        assert!(!g.is_cyclic());                
    }

    #[test]
    fn double_cycle_is_cyclic() {
        use Graph;
        let mut g = Graph::new(4);        
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        g.add_edge(1, 3);
    }

    #[test]
    fn no_edges_is_acyclic() {
        use Graph;
        for n in 0..1000 {
            let g = Graph::new(n);
            assert!(g.is_acyclic());
        }
    }

    #[test]
    fn cycle_is_cyclic() {
        use Graph;
        for n in 3 .. 100 {
            let mut g = Graph::new(n);
            for m in 0 .. (n - 1) {                
                g.add_edge(m, m + 1);
            }
            assert!(g.is_acyclic());
            g.add_edge(n - 1, 0);
            assert!(g.is_cyclic());
        }
    }

    #[test]
    fn forest_is_acyclic() {
        use Graph;
        let mut g = Graph::new(10 * 5);
        for n in 0..10 {
            let m = n * 5;
            g.add_edge(m, m + 1);
            g.add_edge(m + 1, m + 2);
            g.add_edge(m + 2, m + 3);
            g.add_edge(m + 3, m + 4);            
        }
        assert!(g.is_acyclic());
    }
}
