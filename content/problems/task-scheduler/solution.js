/**
 * @param {string[]} tasks
 * @param {number} n
 * @return {number}
 */
var leastInterval = function(tasks, n) {
    const counts = new Array(26).fill(0);
    for (const t of tasks) counts[t.charCodeAt(0) - 65]++;
    const f = Math.max(...counts);
    const k = counts.filter(c => c === f).length;
    return Math.max(tasks.length, (f - 1) * (n + 1) + k);
};
