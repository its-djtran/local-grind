impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n <= 2 {
            return (0..n).collect();
        }
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        let mut degree = vec![0; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }
        let mut leaves: Vec<usize> = (0..n).filter(|&v| degree[v] == 1).collect();
        let mut remaining = n;
        while remaining > 2 {
            remaining -= leaves.len();
            let mut next = Vec::new();
            for &leaf in &leaves {
                for &w in &graph[leaf] {
                    degree[w] -= 1;
                    if degree[w] == 1 {
                        next.push(w);
                    }
                }
            }
            leaves = next;
        }
        leaves.into_iter().map(|v| v as i32).collect()
    }
}
