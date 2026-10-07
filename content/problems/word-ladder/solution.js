/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */
var ladderLength = function(beginWord, endWord, wordList) {
    if (!wordList.includes(endWord)) return 0;
    const L = beginWord.length;
    const buckets = new Map();
    for (const w of wordList) {
        for (let i = 0; i < L; i++) {
            const key = w.slice(0, i) + "*" + w.slice(i + 1);
            if (!buckets.has(key)) buckets.set(key, []);
            buckets.get(key).push(w);
        }
    }
    const visited = new Set([beginWord]);
    let queue = [beginWord];
    let steps = 1;
    while (queue.length) {
        const next = [];
        for (const word of queue) {
            if (word === endWord) return steps;
            for (let i = 0; i < L; i++) {
                const key = word.slice(0, i) + "*" + word.slice(i + 1);
                const list = buckets.get(key);
                if (!list) continue;
                buckets.delete(key);
                for (const w of list) {
                    if (!visited.has(w)) {
                        visited.add(w);
                        next.push(w);
                    }
                }
            }
        }
        queue = next;
        steps++;
    }
    return 0;
};
