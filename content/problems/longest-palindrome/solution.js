/**
 * @param {string} s
 * @return {number}
 */
var longestPalindrome = function(s) {
    const unpaired = new Set();
    for (const ch of s) {
        if (unpaired.has(ch)) unpaired.delete(ch);
        else unpaired.add(ch);
    }
    // Every character except the unpaired ones forms mirrored pairs; one leftover may sit in the middle.
    return s.length - unpaired.size + (unpaired.size > 0 ? 1 : 0);
};
