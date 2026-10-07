The **median** of a sorted list of numbers is its middle element when the list has odd length, and the average of its two middle elements when the length is even. For example, the median of `[2,3,4]` is `3`, and the median of `[2,3]` is `(2 + 3) / 2 = 2.5`.

Design a class `MedianFinder` that receives integers one at a time and can report the median of everything it has received so far:

- `MedianFinder()` creates an empty finder.
- `void addNum(int num)` adds `num` to the collection.
- `double findMedian()` returns the median of all numbers added so far. Answers within `10^-5` of the true median are accepted.

<!-- examples -->

**Constraints:**

- `-10^5 <= num <= 10^5`
- `findMedian` is only called after at least one number has been added.
- At most `5 * 10^4` calls in total are made to `addNum` and `findMedian`.

**Follow-up:**

- If every number in the stream lies in the range `[0, 100]`, how could you make both operations faster?
- If `99%` of the numbers lie in `[0, 100]`, how would you adapt that idea?
