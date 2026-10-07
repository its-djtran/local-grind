/**
 * @param {string} text1
 * @param {string} text2
 * @return {number}
 */
var longestCommonSubsequence = function(text1, text2) {
    if (text1.length < text2.length) [text1, text2] = [text2, text1];
    const m = text2.length;
    let prev = new Int32Array(m + 1), cur = new Int32Array(m + 1);
    for (let i = 0; i < text1.length; i++) {
        const ch = text1.charCodeAt(i);
        for (let j = 0; j < m; j++) {
            cur[j + 1] = ch === text2.charCodeAt(j) ? prev[j] + 1 : Math.max(prev[j + 1], cur[j]);
        }
        [prev, cur] = [cur, prev];
    }
    return prev[m];
};
