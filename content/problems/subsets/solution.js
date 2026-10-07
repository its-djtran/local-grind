/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
    const result = [[]];
    for (const x of nums) {
        const size = result.length;
        for (let i = 0; i < size; i++) result.push([...result[i], x]);
    }
    return result;
};
