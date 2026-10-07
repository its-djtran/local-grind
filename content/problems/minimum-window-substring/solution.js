/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
var minWindow = function(s, t) {
    const need = new Int32Array(128);
    for (let i = 0; i < t.length; i++) need[t.charCodeAt(i)]++;
    let missing = t.length, left = 0;
    let bestStart = 0, bestLen = s.length + 1;
    for (let right = 0; right < s.length; right++) {
        const c = s.charCodeAt(right);
        if (need[c] > 0) missing--;
        need[c]--;
        if (missing === 0) {
            // Drop surplus characters from the left; the window stays valid.
            while (need[s.charCodeAt(left)] < 0) {
                need[s.charCodeAt(left)]++;
                left++;
            }
            if (right - left + 1 < bestLen) {
                bestStart = left;
                bestLen = right - left + 1;
            }
            // Give up the leftmost required character to look for the next window.
            need[s.charCodeAt(left)]++;
            missing++;
            left++;
        }
    }
    return bestLen > s.length ? "" : s.substring(bestStart, bestStart + bestLen);
};
