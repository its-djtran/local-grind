/**
 * @param {string} s
 * @return {number}
 */
var myAtoi = function(s) {
    const INT_MAX = 2 ** 31 - 1, INT_MIN = -(2 ** 31);
    const n = s.length;
    let i = 0;
    while (i < n && s[i] === ' ') i++;
    let sign = 1;
    if (i < n && (s[i] === '+' || s[i] === '-')) {
        if (s[i] === '-') sign = -1;
        i++;
    }
    let value = 0;
    while (i < n && s[i] >= '0' && s[i] <= '9') {
        const d = s.charCodeAt(i) - 48;
        if (value > Math.floor((INT_MAX - d) / 10)) return sign === 1 ? INT_MAX : INT_MIN;
        value = value * 10 + d;
        i++;
    }
    return sign * value;
};
