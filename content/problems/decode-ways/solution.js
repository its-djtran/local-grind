/**
 * @param {string} s
 * @return {number}
 */
var numDecodings = function(s) {
    let prev = 1, cur = s[0] === '0' ? 0 : 1;
    for (let i = 1; i < s.length; i++) {
        let next = 0;
        if (s[i] !== '0') next += cur;
        if (s[i - 1] === '1' || (s[i - 1] === '2' && s[i] <= '6')) next += prev;
        prev = cur;
        cur = next;
    }
    return cur;
};
