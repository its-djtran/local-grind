/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
var characterReplacement = function(s, k) {
    const count = new Int32Array(26);
    let left = 0, maxf = 0;
    for (let right = 0; right < s.length; right++) {
        const c = s.charCodeAt(right) - 65;
        maxf = Math.max(maxf, ++count[c]);
        // The window never shrinks: if it is too costly, slide it right by one instead.
        if (right - left + 1 - maxf > k) {
            count[s.charCodeAt(left) - 65]--;
            left++;
        }
    }
    return s.length - left;
};
