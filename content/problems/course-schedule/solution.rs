impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![Vec::new(); n];
        let mut indegree = vec![0; n];
        for p in &prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            graph[b].push(a);
            indegree[a] += 1;
        }
        let mut queue: VecDeque<usize> = (0..n).filter(|&v| indegree[v] == 0).collect();
        let mut taken = 0;
        while let Some(v) = queue.pop_front() {
            taken += 1;
            for &w in &graph[v] {
                indegree[w] -= 1;
                if indegree[w] == 0 {
                    queue.push_back(w);
                }
            }
        }
        taken == n
    }
}
