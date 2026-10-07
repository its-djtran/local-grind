/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function(digits) {
    if (!digits) return [];
    const keypad = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let combos = [""];
    for (const d of digits) {
        const next = [];
        for (const c of combos)
            for (const letter of keypad[d]) next.push(c + letter);
        combos = next;
    }
    return combos;
};
