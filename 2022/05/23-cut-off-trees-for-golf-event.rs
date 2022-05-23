// https://leetcode.cn/problems/cut-off-trees-for-golf-event/

impl Solution {
  pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
      let mut trees = vec![];
      for i in 0..forest.len() {
          for j in 0..forest[0].len() {
              if forest[i][j] > 1 {
                  trees.push((forest[i][j], i, j));
              }
          }
      }
      trees.sort_by(|a, b| a.0.cmp(&b.0));
      fn distance(start:(usize,usize),end:(usize,usize), forest: &Vec<Vec<i32>>) -> i32 {
          let mut queue = vec![start];
          let mut visited = vec![vec![false; forest[0].len()]; forest.len()];
          visited[start.0][start.1] = true;
          let mut step = 0;
          while !queue.is_empty() {
              let mut new_queue = vec![];
              for (i, j) in queue {
                  if i == end.0 && j == end.1 {
                      return step;
                  }
                  for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                      let ni = i as i32 + di;
                      let nj = j as i32 + dj;
                      if ni >= 0 && ni < forest.len() as i32 && nj >= 0 && nj < forest[0].len() as i32 && !visited[ni as usize][nj as usize] && forest[ni as usize][nj as usize] > 0 {
                          new_queue.push((ni as usize, nj as usize));
                          visited[ni as usize][nj as usize] = true;
                      }
                  }
              }
              queue = new_queue;
              step += 1;
          }
          -1
      }
      let mut ans = 0;
      let mut start = (0, 0);
      for (_h, x, y) in trees {
          let d = distance(start, (x, y), &forest);
          if d == -1 {
              return -1;
          }
          ans += d;
          start = (x, y);
      }
      ans as i32
  }
}