/**
 * @param {string} ransomNote
 * @param {string} magazine
 * @return {boolean}
 */
var canConstruct = function(ransomNote, magazine) {
    if (ransomNote.length > magazine.length) return false;
    const counts = new Array(26).fill(0);
    for (let k = 0; k < magazine.length; k++) {
        counts[magazine.charCodeAt(k) - 97]++;
    }
    for (let k = 0; k < ransomNote.length; k++) {
        const i = ransomNote.charCodeAt(k) - 97;
        if (counts[i] === 0) return false;
        counts[i]--;
    }
    return true;
};
