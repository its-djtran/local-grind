/**
 * @param {string} s
 * @return {boolean}
 */
var isPalindrome = function(s) {
    const isAlnum = (c) =>
        (c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122);
    const lower = (c) => (c >= 65 && c <= 90 ? c + 32 : c);
    let i = 0, j = s.length - 1;
    while (i < j) {
        while (i < j && !isAlnum(s.charCodeAt(i))) i++;
        while (i < j && !isAlnum(s.charCodeAt(j))) j--;
        if (lower(s.charCodeAt(i)) !== lower(s.charCodeAt(j))) return false;
        i++;
        j--;
    }
    return true;
};
