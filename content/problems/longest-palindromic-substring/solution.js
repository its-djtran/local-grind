/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function(s) {
    const t = '^#' + s.split('').join('#') + '#$';
    const m = t.length;
    const radius = new Int32Array(m);
    let center = 0, right = 0, bestLen = 0, bestMid = 0;
    for (let i = 1; i < m - 1; i++) {
        if (i < right) radius[i] = Math.min(right - i, radius[2 * center - i]);
        while (t[i + radius[i] + 1] === t[i - radius[i] - 1]) radius[i]++;
        if (i + radius[i] > right) {
            center = i;
            right = i + radius[i];
        }
        if (radius[i] > bestLen) {
            bestLen = radius[i];
            bestMid = i;
        }
    }
    const start = (bestMid - bestLen) >> 1;
    return s.substring(start, start + bestLen);
};
