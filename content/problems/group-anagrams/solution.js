/**
 * @param {string[]} strs
 * @return {string[][]}
 */
var groupAnagrams = function(strs) {
    const groups = new Map();
    for (const word of strs) {
        const count = new Array(26).fill(0);
        for (let i = 0; i < word.length; i++) count[word.charCodeAt(i) - 97]++;
        const key = count.join(',');
        const g = groups.get(key);
        if (g) g.push(word);
        else groups.set(key, [word]);
    }
    return [...groups.values()];
};
