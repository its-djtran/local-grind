class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> graph(numCourses);
        vector<int> indegree(numCourses, 0);
        for (auto& p : prerequisites) {
            graph[p[1]].push_back(p[0]);
            indegree[p[0]]++;
        }
        queue<int> q;
        for (int v = 0; v < numCourses; v++)
            if (indegree[v] == 0) q.push(v);
        int taken = 0;
        while (!q.empty()) {
            int v = q.front();
            q.pop();
            taken++;
            for (int w : graph[v])
                if (--indegree[w] == 0) q.push(w);
        }
        return taken == numCourses;
    }
};
