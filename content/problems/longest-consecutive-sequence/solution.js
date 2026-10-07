/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function(nums) {
    const present = new Set(nums);
    let best = 0;
    for (const x of present) {
        if (!present.has(x - 1)) {
            let y = x + 1;
            while (present.has(y)) y++;
            if (y - x > best) best = y - x;
        }
    }
    return best;
};
