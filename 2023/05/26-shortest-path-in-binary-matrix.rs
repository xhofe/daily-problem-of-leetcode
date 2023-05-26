// BFS
impl Solution {
  pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
      if grid[0][0] == 1 {
          return -1;
      }
      let mut q = std::collections::VecDeque::new();
      q.push_back((0, 0, 1));
      let mut vis = vec![vec![false; grid.len()]; grid.len()];
      vis[0][0] = true;
      let mut ans = -1;
      let n = grid.len() as i32;
      while let Some((x, y, step)) = q.pop_front() {
          if x == n - 1 && y == n - 1 {
              ans = step;
              break;
          }
          for i in -1..=1 {
              for j in -1..=1 {
                  if i == 0 && j == 0 {
                      continue;
                  }
                  let (nx, ny) = (x + i, y + j);
                  if nx >= 0
                      && nx < n
                      && ny >= 0
                      && ny < n
                      && grid[nx as usize][ny as usize] == 0
                      && !vis[nx as usize][ny as usize]
                  {
                      vis[nx as usize][ny as usize] = true;
                      q.push_back((nx, ny, step + 1));
                  }
              }
          }
      }
      ans
  }
}