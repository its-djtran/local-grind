/**
 */
var TimeMap = function() {
    this.store = new Map();
};

/**
 * @param {string} key
 * @param {string} value
 * @param {number} timestamp
 * @return {void}
 */
TimeMap.prototype.set = function(key, value, timestamp) {
    let entry = this.store.get(key);
    if (!entry) {
        entry = { times: [], values: [] };
        this.store.set(key, entry);
    }
    entry.times.push(timestamp);
    entry.values.push(value);
};

/**
 * @param {string} key
 * @param {number} timestamp
 * @return {string}
 */
TimeMap.prototype.get = function(key, timestamp) {
    const entry = this.store.get(key);
    if (!entry) return '';
    const times = entry.times;
    let lo = 0, hi = times.length;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (times[mid] <= timestamp) lo = mid + 1;
        else hi = mid;
    }
    return lo ? entry.values[lo - 1] : '';
};

/**
 * Your TimeMap object will be instantiated and called as such:
 * var obj = new TimeMap()
 * obj.set(key, value, timestamp)
 * var param_2 = obj.get(key, timestamp)
 */
