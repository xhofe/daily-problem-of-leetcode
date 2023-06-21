impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        fn f(mut chessboard: Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
            let mut ans = 0;
            let dirs = [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ];
            let mut q = std::collections::VecDeque::new();
            q.push_back((x, y));
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                for &(dx, dy) in dirs.iter() {
                    let mut white = vec![];
                    let (mut nx, mut ny) = (x as i32, y as i32);
                    loop {
                        nx += dx;
                        ny += dy;
                        if nx < 0
                            || nx >= chessboard.len() as i32
                            || ny < 0
                            || ny >= chessboard[0].len() as i32
                        {
                            break;
                        }
                        let (nx, ny) = (nx as usize, ny as usize);
                        if chessboard[nx][ny] == b'O' {
                            white.push((nx, ny));
                        } else {
                            break;
                        }
                    }
                    if nx >= 0
                        && nx < chessboard.len() as i32
                        && ny >= 0
                        && ny < chessboard[0].len() as i32
                        && chessboard[nx as usize][ny as usize] == b'X'
                    {
                        for &(x, y) in white.iter() {
                            if !q.contains(&(x, y)) {
                                q.push_back((x, y));
                            }
                            ans += 1;
                            chessboard[x][y] = b'X';
                        }
                    }
                }
            }
            ans
        }
        let mut ans = 0;
        let chessboard = chessboard
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<_>>();
        for i in 0..chessboard.len() {
            for j in 0..chessboard[0].len() {
                if chessboard[i][j] == b'.' {
                    ans = ans.max(f(chessboard.clone(), i, j));
                }
            }
        }
        ans
    }
}
