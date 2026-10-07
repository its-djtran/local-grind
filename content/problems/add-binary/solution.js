/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function(a, b) {
    let i = a.length - 1, j = b.length - 1, carry = 0;
    const digits = [];
    while (i >= 0 || j >= 0 || carry) {
        let total = carry;
        if (i >= 0) total += a.charCodeAt(i--) - 48;
        if (j >= 0) total += b.charCodeAt(j--) - 48;
        digits.push(total & 1 ? '1' : '0');
        carry = total >> 1;
    }
    return digits.reverse().join('');
};
