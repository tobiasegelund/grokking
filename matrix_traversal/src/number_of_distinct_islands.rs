use std::collections::HashSet;

fn distinct_islands(matrix: &mut Vec<Vec<u8>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut islands: HashSet<String> = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            let traversal = traverse_island_dfs(matrix, &mut visited, i, j, "O");
            islands.insert(traversal);
        }
    }

    islands.len().try_into().unwrap()
}

fn traverse_island_dfs(
    matrix: &mut Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    direction: &str,
) -> String {
    if (x >= matrix.len()) || (y >= matrix[0].len()) {
        return "".to_string();
    }

    if (matrix[x][y] == 0) || (visited[x][y]) {
        return "".to_string();
    }

    visited[x][y] = true;

    let mut island_traversal = direction.to_string();
    island_traversal += &traverse_island_dfs(matrix, visited, x + 1, y, "R");
    island_traversal += &traverse_island_dfs(matrix, visited, x, y + 1, "U");
    if x != 0 {
        island_traversal += &traverse_island_dfs(matrix, visited, x - 1, y, "L");
    }
    if y != 0 {
        island_traversal += &traverse_island_dfs(matrix, visited, x, y - 1, "D");
    }

    island_traversal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_islands() {
        let mut matrix: Vec<Vec<u8>> = vec![
            vec![1, 1, 0, 1, 1],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 1, 0, 1],
        ];

        let expected = 3;

        let got = distinct_islands(&mut matrix);

        assert_eq!(got, expected);
    }
}
