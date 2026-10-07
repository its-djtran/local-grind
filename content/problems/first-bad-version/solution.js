/**
 * Definition for isBadVersion()
 *
 * @param {integer} version number
 * @return {boolean} whether the version is bad
 * isBadVersion = function(version) {
 *     ...
 * };
 */

/**
 * @param {function} isBadVersion()
 * @return {function}
 */
var solution = function(isBadVersion) {
    /**
     * @param {number} n
     * @return {number}
     */
    return function(n) {
        let lo = 1, hi = n;
        while (lo < hi) {
            const mid = lo + Math.floor((hi - lo) / 2);
            if (isBadVersion(mid)) hi = mid; // mid could be the first bad version
            else lo = mid + 1;               // everything up to mid is good
        }
        return lo;
    };
};
