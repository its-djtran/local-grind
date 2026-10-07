You are given an integer array `height` of length `n`. Picture `n` vertical lines on the x-axis: line `i` stands at `x = i` and reaches up to height `height[i]`.

Choose two of the lines. Together with the x-axis they form a container, and the water it can hold is the distance between the lines times the height of the **shorter** line. Return the largest amount of water any pair of lines can hold.

The container cannot be tilted.

<!-- examples -->

**Constraints:**

- `n == height.length`
- `2 <= n <= 10^5`
- `0 <= height[i] <= 10^4`
