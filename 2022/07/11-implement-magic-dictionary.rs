// https://leetcode.cn/problems/implement-magic-dictionary/

struct MagicDictionary {
    dictionary: Vec<String>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        Self {
            dictionary: vec![]
        }
    }
    
    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.dictionary = dictionary;
    }
    
    fn search(&self, search_word: String) -> bool {
        for word in &self.dictionary {
            if word.len() != search_word.len() {
                continue;
            }
            let mut diff = 0;
            for i in 0..word.len() {
                if word.as_bytes()[i] != search_word.as_bytes()[i] {
                    diff += 1;
                    if diff > 1 {
                        break;
                    }
                }
            }
            if diff == 1 {
                return true;
            }
        }
        false
    }
}


/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
