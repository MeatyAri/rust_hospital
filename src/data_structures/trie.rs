use crate::data_structures::linked_list::LinkedList;

#[derive(Default)]
pub struct TrieNode {
    pub children: [Option<Box<TrieNode>>; 26],
    pub word_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            word_end: false,
        }
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, key: &str) {
        let mut curr = &mut self.root;
        for c in key.chars() {
            let index = (c as usize) - ('a' as usize);
            if curr.children[index].is_none() {
                curr.children[index] = Some(Box::new(TrieNode::new()));
            }
            curr = curr.children[index].as_mut().unwrap();
        }
        curr.word_end = true;
    }

    pub fn search(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for c in key.chars() {
            let index = (c as usize) - ('a' as usize);
            if curr.children[index].is_none() {
                return false;
            }
            curr = curr.children[index].as_ref().unwrap();
        }
        curr.word_end
    }

    pub fn auto_complete(&self, prefix: &str) -> LinkedList<String> {
        let mut curr = &self.root;
        for c in prefix.chars() {
            let index = (c as usize) - ('a' as usize);
            if curr.children[index].is_none() {
                return LinkedList::new();
            }
            curr = curr.children[index].as_ref().unwrap();
        }
        let mut results = LinkedList::new();
        Self::collect_words(curr, prefix.to_string(), &mut results);
        results
    }

    pub fn collect_words(node: &TrieNode, prefix: String, results: &mut LinkedList<String>) {
        if node.word_end {
            results.push_front(prefix.clone());
        }
        for (i, child) in node.children.iter().enumerate() {
            if let Some(child_node) = child {
                let next_char = (i as u8 + b'a') as char;
                Self::collect_words(child_node, format!("{}{}", prefix, next_char), results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();
        let words = ["and", "ant", "do", "geek", "dad", "ball"];
        for word in &words {
            trie.insert(word);
        }

        assert!(trie.search("and"));
        assert!(trie.search("ant"));
        assert!(trie.search("do"));
        assert!(trie.search("geek"));
        assert!(trie.search("dad"));
        assert!(trie.search("ball"));
        assert!(!trie.search("bat"));
        assert!(!trie.search("ge"));
    }

    #[test]
    fn test_auto_complete() {
        let mut trie = Trie::new();
        let words = ["and", "ant", "do", "geek", "dad", "ball"];
        for word in words {
            trie.insert(word);
        }

        let prefix = "ge";
        let auto_complete_results = trie.auto_complete(prefix);
        let expected_results: Vec<String> = vec!["geek".to_string()];
        let result_vec: Vec<String> = auto_complete_results.iter().cloned().collect();
        assert_eq!(result_vec.into_iter().collect::<std::collections::HashSet<_>>(), expected_results.into_iter().collect::<std::collections::HashSet<_>>());

        let prefix = "a";
        let auto_complete_results = trie.auto_complete(prefix);
        let expected_results: Vec<String> = vec!["and".to_string(), "ant".to_string()];
        let result_vec: Vec<String> = auto_complete_results.iter().cloned().collect();
        assert_eq!(result_vec.into_iter().collect::<std::collections::HashSet<_>>(), expected_results.into_iter().collect::<std::collections::HashSet<_>>());

        let prefix = "d";
        let auto_complete_results = trie.auto_complete(prefix);
        let expected_results: Vec<String> = vec!["dad".to_string(), "do".to_string()];
        let result_vec: Vec<String> = auto_complete_results.iter().cloned().collect();
        assert_eq!(result_vec.into_iter().collect::<std::collections::HashSet<_>>(), expected_results.into_iter().collect::<std::collections::HashSet<_>>());

        let prefix = "b";
        let auto_complete_results = trie.auto_complete(prefix);
        let expected_results: Vec<String> = vec!["ball".to_string()];
        let result_vec: Vec<String> = auto_complete_results.iter().cloned().collect();
        assert_eq!(result_vec.into_iter().collect::<std::collections::HashSet<_>>(), expected_results.into_iter().collect::<std::collections::HashSet<_>>());
    }
}