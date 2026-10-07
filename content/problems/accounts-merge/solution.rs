impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        let mut id: HashMap<&str, usize> = HashMap::new();
        let mut emails: Vec<&str> = Vec::new();
        let mut owner: Vec<usize> = Vec::new();
        let mut parent: Vec<usize> = Vec::new();
        for (a, acc) in accounts.iter().enumerate() {
            for e in &acc[1..] {
                if !id.contains_key(e.as_str()) {
                    id.insert(e, emails.len());
                    parent.push(emails.len());
                    emails.push(e);
                    owner.push(a);
                }
            }
            let root = find(&mut parent, id[acc[1].as_str()]);
            for e in &acc[2..] {
                let other = find(&mut parent, id[e.as_str()]);
                if other != root {
                    parent[other] = root;
                }
            }
        }
        let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
        for e in 0..emails.len() {
            let r = find(&mut parent, e);
            groups.entry(r).or_default().push(emails[e]);
        }
        groups
            .into_iter()
            .map(|(root, mut list)| {
                list.sort_unstable();
                let mut row = Vec::with_capacity(list.len() + 1);
                row.push(accounts[owner[root]][0].clone());
                row.extend(list.into_iter().map(String::from));
                row
            })
            .collect()
    }
}
