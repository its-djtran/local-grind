class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n <= 2) {
            vector<int> res(n);
            iota(res.begin(), res.end(), 0);
            return res;
        }
        vector<vector<int>> graph(n);
        vector<int> degree(n, 0);
        for (auto& e : edges) {
            graph[e[0]].push_back(e[1]);
            graph[e[1]].push_back(e[0]);
            degree[e[0]]++;
            degree[e[1]]++;
        }
        vector<int> leaves;
        for (int v = 0; v < n; v++)
            if (degree[v] == 1) leaves.push_back(v);
        int remaining = n;
        while (remaining > 2) {
            remaining -= leaves.size();
            vector<int> next;
            for (int leaf : leaves)
                for (int w : graph[leaf])
                    if (--degree[w] == 1) next.push_back(w);
            leaves = move(next);
        }
        return leaves;
    }
};
