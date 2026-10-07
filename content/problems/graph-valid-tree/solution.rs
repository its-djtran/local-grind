impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() + 1 != n {
            return false;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for e in &edges {
            let ra = find(&mut parent, e[0] as usize);
            let rb = find(&mut parent, e[1] as usize);
            if ra == rb {
                return false;
            }
            parent[ra] = rb;
        }
        true
    }
}
