#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_docs)]

//! Key-value collection to make fuzzy searches

#[cfg(test)]
mod tests;
mod branch;
mod inserter;
mod collector;

use std::slice::Iter;
use std::sync::Arc;
use levenshtein_automata::LevenshteinAutomatonBuilder;
use branch::Node;
pub use inserter::Inserter;
pub use collector::Collector;


/// FuzzyTrie is a trie with a LevensteinAutomata to make fuzzy searches
/// 
/// # Example
/// 
/// ```
/// use fuzzy_trie::FuzzyTrie;
/// 
/// let mut trie = FuzzyTrie::new(2, false);
/// trie.insert("vanilla").insert("vanilla item");
/// trie.insert("hello").insert("hello item");
/// trie.insert("helo").insert("helo item");
/// trie.insert("vanllo").insert("vanllo item");
///
///
/// let mut hello = Vec::new();
/// trie.fuzzy_search("hello", &mut hello);
/// let mut hello_iter = hello.into_iter();
///
/// assert_eq!(hello_iter.next(), Some((0, &"hello item")));
/// assert_eq!(hello_iter.next(), Some((1, &"helo item")));
/// assert_eq!(hello_iter.next(), None);
///
///
/// let mut vanila = Vec::new();
/// trie.fuzzy_search("vanilla", &mut vanila);
/// let mut vanila_iter = vanila.into_iter();
///
/// assert_eq!(vanila_iter.next(), Some((0, &"vanilla item")));
/// assert_eq!(vanila_iter.next(), Some((2, &"vanllo item")));
/// assert_eq!(vanila_iter.next(), None);
/// ```
/// 
#[derive(Clone)]
pub struct FuzzyTrie<T> {
    values: Vec<T>,
    root: Node,
    dfa_builder: Arc<LevenshteinAutomatonBuilder>,
}


impl<T> FuzzyTrie<T> {
    /// Creates new fuzzy trie and Levenshtein automaton builder
    /// with given max_distance and dameru params
    #[inline]
    pub fn new(max_distance: u8, damerau: bool) -> Self {
        let dfa_builder = LevenshteinAutomatonBuilder::new(max_distance, damerau);
        Self::with_automaton_builder(Arc::from(dfa_builder))
    }


    /// Creates new fuzzy trie with yours Levenshtein automaton builder
    #[inline]
    pub fn with_automaton_builder(dfa_builder: Arc<LevenshteinAutomatonBuilder>) -> Self {
        let values = Vec::new();
        let root = Node::new_branch('\0');
        Self{values, root, dfa_builder}
    }


    /// Inserts value to trie
    /// Returns inserter, to make possible using a value field as key
    /// See `Inserter` for additional information
    #[inline]
    pub fn insert<'a>(&'a mut self, key: &str) -> Inserter<'a, T> {
        self.root.insert(&mut self.values, key)
    }


    /// Makes fuzzy search with given key and puts result to out collector
    /// See `Collector` for additional information
    #[inline]
    pub fn fuzzy_search<'a, C: Collector<'a, T>>(&'a self, key: &str, out: &mut C) {
        let branches = match &self.root {
            Node::Branch(_, branches) => branches,
            _ => unreachable!(),
        };   
        let dfa = self.dfa_builder.build_dfa(key);
        for br in branches {
            br.fuzzy_search(&self.values, &dfa, dfa.initial_state(), out);
        }
    }


    /// Iterator over values
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.values.iter()
    }


    /// Destructs self into inner vec of values
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_value(self) -> Vec<T> {
        self.values
    }


    /// Len of inner values vector
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }
}