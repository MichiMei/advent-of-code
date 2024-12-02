/*use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::Add;
use std::slice::{Iter, IterMut};
use num::One;
//use crate::graph::node::Node;

pub trait WeightRequirements: Copy + Eq + Default + Debug {}
impl<T: Copy + Eq + Default + Debug> WeightRequirements for T {}

pub trait EdgeWeight: WeightRequirements {
    type Weight;
    fn get_weight(&self) -> Self::Weight;
}
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Unweighted ();
impl EdgeWeight for Unweighted {
    type Weight = usize;
    fn get_weight(&self) -> Self::Weight {
        1
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Weighted<W: Copy + Eq + Default + Debug> {
    weight: W,
}
impl<W: WeightRequirements> Weighted<W> {
    pub fn from(weight: W) -> Weighted<W> {
        Self {
            weight,
        }
    }
}
impl<W: Copy + Eq + Default + Debug + WeightRequirements> EdgeWeight for Weighted<W> {
    type Weight = W;
    fn get_weight(&self) -> Self::Weight {
        self.weight
    }
}

pub trait Direction {}
#[derive(Default)]
pub struct Directed<EW: EdgeWeight> {
    rev_edges: HashMap<usize, EW>,
}
impl<EW: EdgeWeight> Direction for Directed<EW> {}
pub struct Undirected ();
impl Direction for Undirected {}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Graph<D: Direction, EW: EdgeWeight> {
    nodes: Vec<Node<D, EW>>,
    edge_count: usize,
}

pub type SimpleGraph = Graph<Undirected, Unweighted>;
pub type DirectedGraph = Graph<Directed<Unweighted>, Unweighted>;
pub type WeightedGraph<W> = Graph<Undirected, Weighted<W>>;
pub type WeightedDirectedGraph<W> = Graph<Directed<Weighted<W>>, Weighted<W>>;

/// Contains implementation for all kinds of Graphs
/// (directed and undirected, weighted and unweighted)
impl<D: Direction, EW: EdgeWeight> Graph<D, EW> {
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn is_node(&self, node: usize) -> bool {
        node < self.get_node_count()
    }

    pub fn get_node(&self, id: usize) -> Option<&Node<D, EW>> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node<D, EW>> {
        self.nodes.get_mut(id)
    }

    pub fn iter(&self) -> NodeIter<D, EW> {
        NodeIter {
            iter: self.nodes.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> NodeIterMut<D, EW> {
        NodeIterMut {
            iter: self.nodes.iter_mut(),
        }
    }
}

/// Contains implementations for all kinds of unweighted Graphs (directed and undirected)
impl<D: Direction> Graph<D, Unweighted> {
    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        if !self.is_node(from) {
            return false
        }
        self.nodes[from].has_edge(to)
    }
}

/// Contains implementations for all kinds of weighted Graphs (directed and undirected)
impl<D: Direction, W: WeightRequirements> Graph<D, Weighted<W>> {
    pub fn get_edge(&self, from: usize, to: usize) -> Option<W> {
        if !self.is_node(from) {
            return None
        }
        self.nodes[from].get_edge(to)
    }
}

/// Contains implementations for all kinds of undirected Graphs (weighted and unweighted)
impl<EW: EdgeWeight> Graph<Undirected, EW> {
    pub fn remove_node(&mut self, id: usize) {
        if let Some(node) = self.get_node(id) {
            for neighbor in node.neighbor_iter() {
                let x = self.get_node(neighbor).unwrap();
                x.add_edge();
            }
        }

        todo!()
    }
}

/// Contains implementations for all kinds of directed Graphs (weighted and unweighted)
impl<EW: EdgeWeight> Graph<Directed<EW>, EW> {

}

/// Contains implementations for unweighted and undirected Graphs
impl SimpleGraph {

}

/// Contains implementations for weighted and undirected Graphs
impl<W: WeightRequirements + Add + One> Graph<Undirected, Weighted<W>> {
    pub fn simplify(&mut self) {
        let mut queue = VecDeque::from_iter(0..self.get_node_count());
        while let Some(current) = queue.pop_front() {
            let current_node = self.get_node(current).unwrap();
            if let Some((n0, n1, weight)) = current_node.simplifiable() {

            }

        }

        todo!()
    }
}
impl<W: WeightRequirements + One> From<Graph<Undirected, Unweighted>>
for Graph<Undirected, Weighted<W>> {
    fn from(mut value: Graph<Undirected, Unweighted>) -> Self {
        let nodes = value.nodes.into_iter()
            .map(|node|
                Node::<Undirected, Weighted<W>>::from(node))
            .collect::<Vec<_>>();
        Self {
            nodes,
            edge_count: value.edge_count,
        }
    }
}


/// Contains implementations for weighted and directed Graphs
impl<W: WeightRequirements> WeightedDirectedGraph<W> {

}
impl<W: WeightRequirements + One> From<Graph<Directed<Unweighted>, Unweighted>>
for WeightedDirectedGraph<W> {
    fn from(mut value: Graph<Directed<Unweighted>, Unweighted>) -> Self {
        let nodes = value.nodes.into_iter()
            .map(|node|
                Node::<Directed<Weighted<W>>, Weighted<W>>::from(node))
            .collect::<Vec<_>>();
        Self {
            nodes,
            edge_count: value.edge_count,
        }
    }
}

pub struct NodeIter<'a, D: Direction, EW: EdgeWeight> {
    iter: Iter<'a, Node<D, EW>>,
}
impl<'a, D: Direction, EW: EdgeWeight> Iterator for NodeIter<'a, D, EW> {
    type Item = &'a Node<D, EW>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct NodeIterMut<'a, D: Direction, EW: EdgeWeight> {
    iter: IterMut<'a, Node<D, EW>>,
}
impl<'a, D: Direction, EW: EdgeWeight> Iterator for NodeIterMut<'a, D, EW> {
    type Item = &'a mut Node<D, EW>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub mod node {
    use std::collections::hash_map::Iter;
    use std::collections::HashMap;
    use std::ops::Add;
    use num::{One, Zero};
    use crate::graph::{Directed, Direction, EdgeWeight, Undirected, Unweighted, Weighted, WeightRequirements};

    #[derive(Default, Debug, Eq, PartialEq, Clone)]
    pub struct Node<D: Direction, W: EdgeWeight> {
        id: usize,
        edges: HashMap<usize, W>,
        direction: D,
    }

    /// Contains implementations for all kinds of nodes
    /// (directed and undirected, weighted and unweighted)
    impl<D: Direction, EW: EdgeWeight> Node<D, EW> {
        pub fn new(id: usize, direction: D) -> Self {
            Self {
                id,
                edges: HashMap::new(),
                direction,
            }
        }

        pub fn get_edge_count(&self) -> usize {
            self.edges.len()
        }

        pub fn neighbor_iter(&self) -> NeighborIterator<EW> {
            NeighborIterator {
                iter: self.edges.iter(),
            }
        }
    }

    /// Contains implementations for all kinds of undirected nodes (weighted and unweighted)
    impl<EW: EdgeWeight> Node<Undirected, EW> {

    }

    /// Contains implementations for all kinds of directed nodes (weighted and unweighted)
    impl<EW: EdgeWeight> Node<Directed<EW>, EW> {
        pub fn incoming_neighbor_iter(&self) -> NeighborIterator<EW> {
            NeighborIterator {
                iter: self.direction.rev_edges.iter(),
            }
        }
    }

    /// Contains implementations for all kinds of unweighted nodes (directed and undirected)
    impl<D: Direction> Node<D, Unweighted> {
        pub fn has_edge(&self, to: usize) -> bool {
            self.edges.contains_key(&to)
        }

        pub fn add_edge(&mut self, to: usize) -> bool {
            self.edges.insert(to, Default::default()).is_none()
        }

        pub fn has_self_edge(&self) -> bool {
            self.edges.contains_key(&self.id)
        }
    }

    /// Contains implementations for all kinds of weighted nodes (directed and undirected)
    impl<D: Direction, W: WeightRequirements> Node<D, Weighted<W>> {
        pub fn get_edge(&self, to: usize) -> Option<W> {
            self.edges.get(&to).copied().map(|w| w.get_weight())
        }

        pub fn add_edge(&mut self, to: usize, weight: W) -> Option<W> {
            self.edges.insert(to, Weighted::from(weight)).map(|w| w.get_weight())
        }

        pub fn edge_iter(&self) -> WeightedEdgeIterator<W> {
            WeightedEdgeIterator {
                iter: self.edges.iter(),
            }
        }

        pub fn get_self_edge(&self) -> Option<W> {
            self.edges.get(&self.id).map(|w| w.get_weight())
        }
    }

    /// Contains implementations for directed and weighted nodes
    impl<W: WeightRequirements> Node<Directed<Weighted<W>>, Weighted<W>> {
        pub fn get_incoming_edge_count(&self) -> usize {
            self.direction.rev_edges.len()
        }

        pub fn get_incoming_edge(&self, from: usize) -> Option<W> {
            self.direction.rev_edges.get(&from).copied().map(|w| w.get_weight())
        }

        pub fn add_incoming_edge(&mut self, from: usize, weight: W) -> Option<W> {
            self.direction.rev_edges.insert(from, Weighted::from(weight))
                .map(|w| w.get_weight())
        }

        pub fn incoming_edge_iter(&self) -> WeightedEdgeIterator<W> {
            WeightedEdgeIterator {
                iter: self.direction.rev_edges.iter(),
            }
        }
    }
    impl<W: WeightRequirements + Add + Zero> Node<Directed<Weighted<W>>, Weighted<W>> {

    }
    impl<W: WeightRequirements + One> From<Node<Directed<Unweighted>, Unweighted>>
    for Node<Directed<Weighted<W>>, Weighted<W>> {
        fn from(value: Node<Directed<Unweighted>, Unweighted>) -> Self {
            let mut res =
                Self::new(value.id, Default::default());
            for (to, _) in value.edges.iter() {
                res.add_edge(*to, W::one());
            }
            for (from, _) in value.direction.rev_edges.iter() {
                res.add_incoming_edge(*from, W::one());
            }
            res
        }
    }

    /// Contains implementations for undirected and weighted nodes
    impl<W: WeightRequirements> Node<Undirected, Weighted<W>> {

    }
    impl<W: WeightRequirements + Add<Output = W> + Zero> Node<Undirected, Weighted<W>> {
        pub fn simplifiable(&self) -> Option<(usize, usize, W)> {
            let self_edge = self.get_self_edge();
            if self_edge.is_some() && self_edge.unwrap() != W::zero() {
                return None
            }
            let edges = self.edges.iter()
                .filter(|(to, _)| **to != self.id)
                .collect::<Vec<_>>();
            if edges.len() != 2 {
                return None
            }
            Some((*edges[0].0, *edges[1].0, edges[0].1.get_weight() + edges[1].1.get_weight()))
        }
    }
    impl<W: WeightRequirements + One> From<Node<Undirected, Unweighted>>
    for Node<Undirected, Weighted<W>> {
        fn from(value: Node<Undirected, Unweighted>) -> Self {
            let mut res = Self::new(value.id, value.direction);
            for (to, _) in value.edges.iter() {
                res.add_edge(*to, W::one());
            }
            res
        }
    }

    /// Contains implementations for undirected and unweighted nodes
    impl Node<Directed<Unweighted>, Unweighted> {
        pub fn has_incoming_edge(&self, from: usize) -> bool {
            self.direction.rev_edges.contains_key(&from)
        }

        pub fn add_incoming_edge(&mut self, from: usize) -> bool {
            self.direction.rev_edges.insert(from, Default::default()).is_none()
        }
    }

    pub struct WeightedEdgeIterator<'a, W: WeightRequirements> {
        iter: Iter<'a, usize, Weighted<W>>,
    }
    impl<'a, W: WeightRequirements> Iterator for WeightedEdgeIterator<'a, W> {
        type Item = (usize, W);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
                .map(|(to, weight)| (*to, weight.get_weight()))
        }
    }

    pub struct NeighborIterator<'a, EW: EdgeWeight> {
        iter: Iter<'a, usize, EW>,
    }
    impl<'a, EW: EdgeWeight> Iterator for NeighborIterator<'a, EW> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next()
                .map(|(to, _)| *to)
        }
    }
}
pub mod path_finding {
    use std::collections::VecDeque;
    use std::ops::Add;
    use num::{Unsigned, Zero};
    use crate::graph::{Direction, Graph, WeightRequirements, Unweighted, Weighted, EdgeWeight};

    #[derive(Default, Debug, Eq, PartialEq, Clone)]
    pub struct Path<W: WeightRequirements> {
        weight: W,
        nodes: Vec<usize>,
    }

    #[derive(Default, Debug, Eq, PartialEq, Clone)]
    pub struct SingleSourceShortestPaths<W: WeightRequirements> {
        weights: Vec<Option<W>>,
        prev_nodes: Vec<Option<usize>>,
    }

    impl<W: WeightRequirements> SingleSourceShortestPaths<W> {
        pub fn new() -> SingleSourceShortestPaths<W> {
            Default::default()
        }

        pub fn with_capacity(capacity: usize) -> SingleSourceShortestPaths<W> {
            Self {
                weights: vec![None; capacity],
                prev_nodes: vec![None; capacity],
            }
        }

        pub fn get_path(&self, to: usize) -> Option<Path<W>> {
            if to >= self.weights.len() {
                return  None
            }
            let weight = self.weights[to]?;
            let mut nodes = vec![to];
            let mut current = to;
            while let Some(prev) = self.prev_nodes[current] {
                nodes.push(prev);
                current = prev;
            }
            nodes.reverse();
            Some(Path {
                weight,
                nodes,
            })
        }

        pub fn resize(&mut self, new_capacity: usize) {
            self.prev_nodes.resize(new_capacity, None);
            self.weights.resize(new_capacity, None);
        }

        pub fn set_reachable(&mut self, node: usize, prev_node: Option<usize>, weight: W)
            -> Option<(Option<usize>, W)>
        {
            if node >= self.weights.len() {
                self.resize(node+1);
            }
            let old_node = self.prev_nodes[node];
            let old_weight = self.weights[node];
            self.prev_nodes[node] = prev_node;
            self.weights[node] = Some(weight);
            if let Some(weight) = old_weight {
                Some((old_node, weight))
            } else {
                assert!(old_node.is_none());
                None
            }

        }

        pub fn is_visited(&self, node: usize) -> bool {
            if node >= self.weights.len() {
                return false
            }
            self.weights[node].is_some()
        }
    }

    trait Dijkstra {
        type EW: EdgeWeight<Weight = Self::W>;
        type W: WeightRequirements;
        type D: Direction;
        fn dijkstra_target(graph: &Graph<Self::D, Self::EW>, source: usize, target: usize)
            -> Option<Path<Self::W>>;
        fn dijkstra(graph: &Graph<Self::D, Self::EW>, source: usize)
            -> SingleSourceShortestPaths<Self::W>;
    }

    impl<D: Direction> Dijkstra for Graph<D, Unweighted> {
        type EW = Unweighted;
        type W = usize;
        type D = D;
        fn dijkstra_target(graph: &Graph<Self::D, Self::EW>, source: usize, target: usize)
            -> Option<Path<Self::W>>
        {
            let mut paths =
                SingleSourceShortestPaths::with_capacity(graph.get_node_count());
            let mut queue = VecDeque::new();
            queue.push_back((source, None, 0usize));

            while let Some((current, prev, length)) = queue.pop_front() {
                paths.set_reachable(current, prev, length);
                if current == target {
                    break
                }
                graph.add_unvisited_neighbors_to_queue(
                    current,
                    length,
                    &paths,
                    &mut queue
                );
            }
            paths.get_path(target)
        }
        fn dijkstra(graph: &Graph<Self::D, Self::EW>, source: usize)
            -> SingleSourceShortestPaths<Self::W>
        {
            let mut paths =
                SingleSourceShortestPaths::with_capacity(graph.get_node_count());
            let mut queue = VecDeque::new();
            queue.push_back((source, None, 0usize));

            while let Some((current, prev, length)) = queue.pop_front() {
                paths.set_reachable(current, prev, length);
                graph.add_unvisited_neighbors_to_queue(
                    current,
                    length,
                    &paths,
                    &mut queue
                );
            }
            paths
        }
    }

    impl<D: Direction> Graph<D, Unweighted> {
        fn add_unvisited_neighbors_to_queue(
            &self,
            current_node: usize,
            current_length: usize,
            paths: &SingleSourceShortestPaths<usize>,
            queue: &mut VecDeque<(usize, Option<usize>, usize)>
        )
        {
            if let Some(node_ref) = self.get_node(current_node) {
                node_ref.neighbor_iter()
                    .filter(|neighbor| !paths.is_visited(*neighbor))
                    .for_each(|neighbor|
                        queue.push_back((neighbor, Some(current_node), current_length+1)));
            }
        }
    }

    impl<D: Direction, W: WeightRequirements + Add + Zero + Unsigned> Dijkstra
    for Graph<D, Weighted<W>> {
        type EW = Weighted<Self::W>;
        type W = W;
        type D = D;
        fn dijkstra_target(graph: &Graph<Self::D, Self::EW>, source: usize, target: usize)
            -> Option<Path<Self::W>>
        {
            let mut paths =
                SingleSourceShortestPaths::with_capacity(graph.get_node_count());
            let mut queue = VecDeque::new();
            queue.push_back((source, None, W::zero()));

            while let Some((current, prev, weight)) = queue.pop_front() {
                paths.set_reachable(current, prev, weight);
                if current == target {
                    break
                }
                graph.add_unvisited_neighbors_to_queue(
                    current,
                    weight,
                    &paths,
                    &mut queue
                );
            }
            paths.get_path(target)
        }
        fn dijkstra(graph: &Graph<Self::D, Self::EW>, source: usize)
            -> SingleSourceShortestPaths<Self::W>
        {
            let mut paths =
                SingleSourceShortestPaths::with_capacity(graph.get_node_count());
            let mut queue = VecDeque::new();
            queue.push_back((source, None, W::zero()));

            while let Some((current, prev, weight)) = queue.pop_front() {
                paths.set_reachable(current, prev, weight);
                graph.add_unvisited_neighbors_to_queue(
                    current,
                    weight,
                    &paths,
                    &mut queue
                );
            }
            paths
        }
    }

    impl<D: Direction, W: WeightRequirements + Add<Output = W>> Graph<D, Weighted<W>> {
        fn add_unvisited_neighbors_to_queue(
            &self,
            current_node: usize,
            current_weight: W,
            paths: &SingleSourceShortestPaths<W>,
            queue: &mut VecDeque<(usize, Option<usize>, W)>
        )
        {
            if let Some(node_ref) = self.get_node(current_node) {
                node_ref.edge_iter()
                    .filter(|(neighbor, _)|
                        !paths.is_visited(*neighbor))
                    .for_each(|(neighbor, edge_weight)|
                        queue.push_back(
                            (neighbor, Some(current_node), current_weight + edge_weight)
                        ));
            }
        }
    }

}
*/