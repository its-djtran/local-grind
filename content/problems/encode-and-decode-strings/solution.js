/**
 * @param {string[]} strs
 * @return {string}
 */
var encode = function(strs) {
    return strs.map((s) => s.length + '#' + s).join('');
};

/**
 * @param {string} s
 * @return {string[]}
 */
var decode = function(s) {
    const out = [];
    let i = 0;
    while (i < s.length) {
        const j = s.indexOf('#', i);
        const n = parseInt(s.substring(i, j), 10);
        out.push(s.substr(j + 1, n));
        i = j + 1 + n;
    }
    return out;
};

/**
 * Your functions will be called as such:
 * decode(encode(input));
 */
