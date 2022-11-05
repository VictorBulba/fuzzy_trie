use crate::FuzzyTrie;
use crate::Config;
use crate::LevenshteinConfig;


#[test]
fn test() {
    let mut index = FuzzyTrie::new(2, false);
    let s = "жфг";
    index.insert(s).insert(234);
    let mut v: Vec<usize> = Vec::new();
    index.fuzzy_search(s, &mut v);
    assert_eq!(Some(234), v.into_iter().next());
}


#[test]
fn test_custom_config() {
    let default = LevenshteinConfig{distance: 2, damerau: false};
    let len4 = LevenshteinConfig{distance: 1, damerau: false};
    let len3 = LevenshteinConfig{distance: 0, damerau: false};
    let config = Config{default, other: vec![(len3, 3), (len4, 4)]};
    let mut index = FuzzyTrie::new_with_config(&config);
    
    index.insert("key").insert(10);
    index.insert("kry").insert(15);
    index.insert("tick").insert(20);
    index.insert("tack").insert(25);
    index.insert("takk").insert(30);
    index.insert("hello").insert(35);
    index.insert("hollo").insert(40);
    index.insert("holmo").insert(45);
    index.insert("hommo").insert(50);

    let mut key: Vec<usize> = Vec::new();
    index.fuzzy_search("key", &mut key);
    let mut key_iter = key.into_iter();
    assert_eq!(Some(10), key_iter.next());
    assert_eq!(None, key_iter.next());

    let mut tick: Vec<usize> = Vec::new();
    index.fuzzy_search("tick", &mut tick);
    let mut tick_iter = tick.into_iter();
    assert_eq!(Some(20), tick_iter.next());
    assert_eq!(Some(25), tick_iter.next());
    assert_eq!(None, tick_iter.next());

    let mut hello: Vec<usize> = Vec::new();
    index.fuzzy_search("hello", &mut hello);
    let mut hello_iter = hello.into_iter();
    assert_eq!(Some(35), hello_iter.next());
    assert_eq!(Some(40), hello_iter.next());
    assert_eq!(Some(45), hello_iter.next());
    assert_eq!(None, hello_iter.next());
}