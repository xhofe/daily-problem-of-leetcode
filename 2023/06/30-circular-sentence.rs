impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_whitespace().collect::<Vec<_>>();
        for i in 1..words.len() {
            if words[i-1].as_bytes()[words[i-1].len()-1] != words[i].as_bytes()[0] {
                return false;
            }
        }
        words[0].as_bytes()[0] == words.last().unwrap().as_bytes()[words.last().unwrap().len() - 1]
    }
}
