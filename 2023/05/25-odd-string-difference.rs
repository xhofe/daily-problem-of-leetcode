impl Solution {
  pub fn odd_string(words: Vec<String>) -> String {
      fn diff(word: &str) -> Vec<i32> {
          let bytes = word.as_bytes();
          let mut diff = vec![0; bytes.len() - 1];
          for i in 0..bytes.len() - 1 {
              diff[i] = bytes[i + 1] as i32 - bytes[i] as i32;
          }
          diff
      }
      let mut count = std::collections::HashMap::new();
      let mut dict = std::collections::HashMap::new();
      for word in words {
          let diff = diff(&word);
          *count.entry(diff.clone()).or_insert(0) += 1;
          dict.insert(diff, word);
      }
      for (k,v) in count {
          if v==1{
              return dict.get(&k).unwrap().clone();
          }
      }
      unreachable!()
  }
}