/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
var wordBreak = function(s, wordDict) {
    const words = new Set(wordDict);
    let longest = 0;
    for (const w of wordDict) longest = Math.max(longest, w.length);
    const n = s.length;
    const ok = new Array(n + 1).fill(false);
    ok[0] = true;
    for (let i = 1; i <= n; i++) {
        for (let j = Math.max(0, i - longest); j < i; j++) {
            if (ok[j] && words.has(s.slice(j, i))) {
                ok[i] = true;
                break;
            }
        }
    }
    return ok[n];
};
