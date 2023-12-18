use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash)]
pub struct Node<K, T> {
    pub id: K,
    pub data: T,
}

pub struct Graph<K, T> {
    id_counter: RefCell<usize>,
    nodes: Rc<RefCell<HashMap<K, Rc<Node<K, T>>>>>,
    list: Rc<RefCell<HashMap<K, HashSet<K>>>>,
}

impl<K, T: Hash> Default for Graph<K, T> {
    fn default() -> Self {
        Graph {
            id_counter: RefCell::new(0),
            nodes: Rc::new(RefCell::new(HashMap::new())),
            list: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl<T: Eq + Hash> Graph<usize, T> {
    pub fn add_node(&self, node_data: T) -> Rc<Node<usize, T>> {
        *self.id_counter.borrow_mut() += 1;
        let id = self.id_counter.borrow().clone();
        self.add_node_by_id(id, node_data)
    }
}

impl<K: Eq + Clone + Hash, T: Eq + Hash> Graph<K, T> {
    pub fn new() -> Graph<K, T> {
        Graph {
            ..Default::default()
        }
    }

    pub fn add_node_by_id(&self, id: K, node_data: T) -> Rc<Node<K, T>> {
        let node = Rc::new(Node {
            id: id.clone(),
            data: node_data,
        });
        self.nodes.borrow_mut().insert(id, node.clone());

        node
    }

    pub fn add_edge(&self, from: K, to: K) {
        self.list.borrow_mut().entry(from).or_default().insert(to);
    }

    pub fn add_double_edge(&self, from: K, to: K) {
        self.add_edge(from.clone(), to.clone());
        self.add_edge(to, from);
    }

    pub fn get_node(&self, id: K) -> Option<Rc<Node<K, T>>> {
        self.nodes.borrow().get(&id).cloned()
    }

    pub fn bfs<F>(&self, id: K, mut visit_callback: F)
    where
        F: FnMut(Rc<Node<K, T>>, u64),
    {
        let mut depth = 0;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut next_queue = VecDeque::new();
        let node = self.get_node(id).unwrap();

        queue.push_back(node.clone());
        while let Some(next_node) = queue.pop_front() {
            if !visited.insert(next_node.id.clone()) {
                continue;
            }

            next_queue.extend(self.get_adjacent_nodes(next_node.clone(), &visited));
            visit_callback(next_node.clone(), depth);

            if queue.is_empty() {
                depth += 1;
                queue.append(&mut next_queue);
                next_queue.clear();
            }
        }
    }

    fn get_adjacent_nodes(
        &self,
        node: Rc<Node<K, T>>,
        visited: &HashSet<K>,
    ) -> Vec<Rc<Node<K, T>>> {
        self.list
            .borrow()
            .get(&node.id)
            .unwrap()
            .difference(&visited)
            .map(|id| self.nodes.borrow().get(id).unwrap().clone())
            .collect()
    }
}
