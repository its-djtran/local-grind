class Heap {
    constructor(less) {
        this.a = [];
        this.less = less;
    }
    get size() { return this.a.length; }
    peek() { return this.a[0]; }
    push(x) {
        const a = this.a, less = this.less;
        let i = a.length;
        a.push(x);
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!less(x, a[p])) break;
            a[i] = a[p];
            i = p;
        }
        a[i] = x;
    }
    pop() {
        const a = this.a, less = this.less;
        const top = a[0];
        const last = a.pop();
        const n = a.length;
        if (n > 0) {
            let i = 0;
            while (true) {
                let c = 2 * i + 1;
                if (c >= n) break;
                if (c + 1 < n && less(a[c + 1], a[c])) c++;
                if (!less(a[c], last)) break;
                a[i] = a[c];
                i = c;
            }
            a[i] = last;
        }
        return top;
    }
}

var MedianFinder = function() {
    this.lo = new Heap((a, b) => a > b); // max-heap of the smaller half
    this.hi = new Heap((a, b) => a < b); // min-heap of the larger half
};

/**
 * @param {number} num
 * @return {void}
 */
MedianFinder.prototype.addNum = function(num) {
    this.lo.push(num);
    this.hi.push(this.lo.pop());
    if (this.hi.size > this.lo.size) this.lo.push(this.hi.pop());
};

/**
 * @return {number}
 */
MedianFinder.prototype.findMedian = function() {
    if (this.lo.size > this.hi.size) return this.lo.peek();
    return (this.lo.peek() + this.hi.peek()) / 2;
};
