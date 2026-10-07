/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
    candidates = [...candidates].sort((a, b) => a - b);
    const result = [], combo = [];

    const backtrack = (start, remaining) => {
        if (remaining === 0) {
            result.push([...combo]);
            return;
        }
        for (let i = start; i < candidates.length; i++) {
            const c = candidates[i];
            if (c > remaining) break;
            combo.push(c);
            backtrack(i, remaining - c);
            combo.pop();
        }
    };

    backtrack(0, target);
    return result;
};
