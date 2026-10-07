class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        parent = {}

        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        owner = {}
        for name, *emails in accounts:
            for e in emails:
                if e not in parent:
                    parent[e] = e
                    owner[e] = name
            root = find(emails[0])
            for e in emails[1:]:
                other = find(e)
                if other != root:
                    parent[other] = root
        groups = defaultdict(list)
        for e in parent:
            groups[find(e)].append(e)
        return [[owner[root]] + sorted(emails) for root, emails in groups.items()]
