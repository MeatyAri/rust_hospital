use serde::{Serialize, Deserialize};
use crate::data_structures::linked_list::LinkedList;
use crate::db::entities::UniqueAttribute;
use crate::data_structures::hash_map::HashMap; // Use the custom HashMap

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    Hospital,
    Home,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Object {
    pub name: String,
}

impl UniqueAttribute for Object {
    fn uattr(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub location_type: LocationType,
    pub objects: LinkedList<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, LinkedList<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: String, location_type: LocationType) {
        let node = Node {
            location_type,
            objects: LinkedList::new(),
        };
        self.nodes.insert(id.clone(), node);
        self.edges.insert(id, LinkedList::new());
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.push_front(to);
        }
    }

    pub fn remove_node(&mut self, id: String) {
        self.nodes.remove(&id);
        self.edges.remove(&id);
        for edges in self.edges.values_mut() {
            edges.remove(&id);
        }

    }

    pub fn add_object_to_node(&mut self, node_id: &str, object: Object) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.objects.push_front(object);
        }
    }

    pub fn print_graph(&self) {
        for (node_id, node) in self.nodes.iter() {
            println!("Node ID: {}", node_id);
            println!("  Location Type: {:?}", node.location_type);
            println!("  Objects:");
            for object in node.objects.iter() {
                println!("    - {}", object.name);
            }
            if let Some(edges) = self.edges.get(node_id) {
                println!("  Edges:");
                for edge in edges.iter() {
                    println!("    -> {}", edge);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string(), LocationType::Hospital);
        assert!(graph.nodes.contains_key("node1"));
        assert!(graph.edges.contains_key("node1"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string(), LocationType::Hospital);
        graph.add_node("node2".to_string(), LocationType::Home);
        graph.add_edge("node1".to_string(), "node2".to_string());
        let edges = graph.edges.get("node1").unwrap();
        let head = edges.head.as_ref().unwrap();
        // print
        graph.print_graph();
        assert_eq!(head.value, "node2".to_string());
    }

    #[test]
    fn test_add_object_to_node() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string(), LocationType::Hospital);
        let object = Object { name: "object1".to_string() };
        graph.add_object_to_node("node1", object.clone());
        let node = graph.nodes.get("node1").unwrap();
        let head = node.objects.head.as_ref().unwrap();
        assert_eq!(head.value.name, object.name);
    }
}
