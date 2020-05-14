use super::FuzzyTrie;


#[test]
fn test() {
	let mut index = FuzzyTrie::new(2, false);
    let s = "жфг";
    index.insert(s).insert(234);
    let mut v: Vec<usize> = Vec::new();
	index.fuzzy_search(s, &mut v);
	assert_eq!(Some(234), v.into_iter().next());
}