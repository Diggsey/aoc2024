use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Debug, Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

fn split_first(s: &str) -> Option<(char, &str)> {
    let mut it = s.chars();
    Some((it.next()?, it.as_str()))
}

impl Trie {
    fn insert(&mut self, pattern: &str) {
        let mut node = self;
        for c in pattern.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_leaf = true;
    }
    fn count_matches<'a>(
        &self,
        child: &Self,
        pattern: &'a str,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        let mut count = 0;
        if child.is_leaf {
            if pattern.is_empty() {
                count += 1;
            } else if let Some(&inner_count) = cache.get(pattern) {
                count += inner_count;
            } else {
                let inner_count = self.count_matches(self, pattern, cache);
                cache.insert(pattern, inner_count);
                count += inner_count
            }
        }
        if let Some((head, tail)) = split_first(pattern) {
            if let Some(grandchild) = child.children.get(&head) {
                count += self.count_matches(grandchild, tail, cache);
            }
        }
        count
    }
}

fn main() {
    let (towels_str, patterns_str) = INPUT.split_once("\n\n").unwrap();

    let mut trie = Trie::default();
    for towel in towels_str.split(", ") {
        trie.insert(towel);
    }

    let mut cache = HashMap::new();
    let count = patterns_str
        .lines()
        .map(|pattern| trie.count_matches(&trie, pattern, &mut cache))
        .sum::<usize>();
    println!("{}", count);
}
