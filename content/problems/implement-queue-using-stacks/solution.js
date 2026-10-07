var MyQueue = function() {
    this.inbox = [];
    this.outbox = [];
};

/**
 * @param {number} x
 * @return {void}
 */
MyQueue.prototype.push = function(x) {
    this.inbox.push(x);
};

/**
 * @return {number}
 */
MyQueue.prototype.pop = function() {
    this.peek();
    return this.outbox.pop();
};

/**
 * @return {number}
 */
MyQueue.prototype.peek = function() {
    if (!this.outbox.length) {
        while (this.inbox.length) this.outbox.push(this.inbox.pop());
    }
    return this.outbox[this.outbox.length - 1];
};

/**
 * @return {boolean}
 */
MyQueue.prototype.empty = function() {
    return !this.inbox.length && !this.outbox.length;
};
