/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
    const last = new Int32Array(128).fill(-1);
    let left = 0, best = 0;
    for (let right = 0; right < s.length; right++) {
        const c = s.charCodeAt(right);
        if (last[c] >= left) left = last[c] + 1;
        last[c] = right;
        best = Math.max(best, right - left + 1);
    }
    return best;
};
