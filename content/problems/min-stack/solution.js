var MinStack = function() {
    this.stack = []; // [value, minimum of the stack up to and including this value]
};

/**
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function(val) {
    const currentMin = this.stack.length ? Math.min(val, this.getMin()) : val;
    this.stack.push([val, currentMin]);
};

/**
 * @return {void}
 */
MinStack.prototype.pop = function() {
    this.stack.pop();
};

/**
 * @return {number}
 */
MinStack.prototype.top = function() {
    return this.stack[this.stack.length - 1][0];
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function() {
    return this.stack[this.stack.length - 1][1];
};
