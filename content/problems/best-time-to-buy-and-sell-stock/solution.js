/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    let low = prices[0], best = 0;
    for (const p of prices) {
        if (p < low) low = p;
        else if (p - low > best) best = p - low;
    }
    return best;
};
