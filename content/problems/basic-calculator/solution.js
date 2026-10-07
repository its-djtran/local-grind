/**
 * @param {string} s
 * @return {number}
 */
var calculate = function(s) {
    let total = 0, sign = 1, num = 0;
    const stack = [];
    for (let k = 0; k < s.length; k++) {
        const ch = s[k];
        if (ch >= '0' && ch <= '9') {
            num = num * 10 + (s.charCodeAt(k) - 48);
        } else if (ch === '+' || ch === '-') {
            total += sign * num;
            num = 0;
            sign = ch === '+' ? 1 : -1;
        } else if (ch === '(') {
            stack.push(total, sign);
            total = 0;
            sign = 1;
        } else if (ch === ')') {
            total += sign * num;
            num = 0;
            const prevSign = stack.pop();
            const prev = stack.pop();
            total = prev + prevSign * total;
        }
    }
    return total + sign * num;
};
