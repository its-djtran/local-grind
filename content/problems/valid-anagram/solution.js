/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function(s, t) {
    if (s.length !== t.length) return false;
    const count = new Int32Array(26);
    for (let i = 0; i < s.length; i++) count[s.charCodeAt(i) - 97]++;
    for (let i = 0; i < t.length; i++) {
        if (--count[t.charCodeAt(i) - 97] < 0) return false;
    }
    return true;
};
