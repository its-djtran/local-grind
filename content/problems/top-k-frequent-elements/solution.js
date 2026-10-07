/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var topKFrequent = function(nums, k) {
    const count = new Map();
    for (const x of nums) count.set(x, (count.get(x) || 0) + 1);
    const buckets = Array.from({ length: nums.length + 1 }, () => []);
    for (const [value, c] of count) buckets[c].push(value);
    const result = [];
    for (let c = nums.length; c > 0; c--) {
        for (const value of buckets[c]) {
            result.push(value);
            if (result.length === k) return result;
        }
    }
    return result;
};
