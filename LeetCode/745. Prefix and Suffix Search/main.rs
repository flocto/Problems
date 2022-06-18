pub struct TrieNode {
    children: Vec<Option<TrieNode>>,
    index: i32,
}

impl TrieNode {
    pub fn new() -> Self {
        let mut children = Vec::new();
        for _ in 0..27 {
            children.push(None);
        }
        TrieNode {
            children: children,
            index: 0,
        }
    }
}

struct WordFilter {
    root: TrieNode,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * (This comment is referring to line 35, parameter used to be `&self` and not `&mut self`)
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut root = TrieNode::new();
        for (index, word) in words.iter().enumerate() {
            let mut word = word.chars().collect::<Vec<char>>();
            word.push('{');
            for i in 0..word.len() {
                let mut node = &mut root;
                node.index = index as i32;
                for j in i..2*word.len() - 1 {
                    let k = word[j % word.len()] as usize - 'a' as usize;
                    if node.children[k].is_none() {
                        node.children[k] = Some(TrieNode::new());
                    }
                    node = node.children[k].as_mut().unwrap();
                    node.index = index as i32;
                }
            } 
        }

        WordFilter { root }
    }
    
    fn f(&mut self, prefix: String, suffix: String) -> i32 {
        let mut node = &mut self.root;
        let search = (suffix + &String::from("{") + &prefix).chars().collect::<Vec<char>>();
        for c in search {
            let k = c as usize - 'a' as usize;
            if node.children[k].is_none() {
                return -1;
            }
            node = node.children[k].as_mut().unwrap();
        }
        node.index
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */