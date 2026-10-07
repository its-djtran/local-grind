/**
 * @param {string} s
 * @param {string} p
 * @return {number[]}
 */
var findAnagrams = function(s, p) {
    const m = p.length, n = s.length;
    if (m > n) return [];
    const diff = new Int32Array(26); // window count minus p count, per letter
    for (let i = 0; i < m; i++) diff[p.charCodeAt(i) - 97]--;
    for (let i = 0; i < m; i++) diff[s.charCodeAt(i) - 97]++;
    let bad = 0; // letters whose counts disagree
    for (const d of diff) if (d !== 0) bad++;
    const out = bad === 0 ? [0] : [];
    for (let i = m; i < n; i++) {
        const add = s.charCodeAt(i) - 97, rem = s.charCodeAt(i - m) - 97;
        if (add !== rem) {
            let d = diff[add]++;
            if (d === 0) bad++; else if (d === -1) bad--;
            d = diff[rem]--;
            if (d === 0) bad++; else if (d === 1) bad--;
        }
        if (bad === 0) out.push(i - m + 1);
    }
    return out;
};
