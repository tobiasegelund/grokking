fn count_islands_dfs(matrix: &mut Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut total_islands = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 1 {
                total_islands += 1;
                visit_island_dfs(matrix, i, j);
            }
        }
    }

    total_islands
}

fn visit_island_dfs(matrix: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if x < 0 as usize || y < 0 as usize {
        return;
    }

    if x >= rows || y >= cols {
        return;
    }

    if matrix[x][y] == 0 {
        return;
    }

    matrix[x][y] = 0;

    visit_island_dfs(matrix, x + 1, y);
    visit_island_dfs(matrix, x, y + 1);
    if x != 0 {
        visit_island_dfs(matrix, x - 1, y);
    }
    if y != 0 {
        visit_island_dfs(matrix, x, y - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_islands_dfs() {
        let mut matrix = vec![
            vec![1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1],
        ];

        let expected: i32 = 6;

        let got = count_islands_dfs(&mut matrix);

        assert_eq!(expected, got);
    }
}
