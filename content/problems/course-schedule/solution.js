/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {boolean}
 */
var canFinish = function(numCourses, prerequisites) {
    const graph = Array.from({ length: numCourses }, () => []);
    const indegree = new Array(numCourses).fill(0);
    for (const [a, b] of prerequisites) {
        graph[b].push(a);
        indegree[a]++;
    }
    const queue = [];
    for (let v = 0; v < numCourses; v++) if (indegree[v] === 0) queue.push(v);
    for (let head = 0; head < queue.length; head++) {
        for (const w of graph[queue[head]]) {
            if (--indegree[w] === 0) queue.push(w);
        }
    }
    return queue.length === numCourses;
};
