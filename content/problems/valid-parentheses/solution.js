/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function(s) {
    const pairs = { ')': '(', ']': '[', '}': '{' };
    const stack = [];
    for (const ch of s) {
        if (ch in pairs) {
            if (stack.length === 0 || stack.pop() !== pairs[ch]) return false;
        } else {
            stack.push(ch);
        }
    }
    return stack.length === 0;
};
