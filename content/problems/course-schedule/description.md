There are `numCourses` courses, numbered `0` through `numCourses - 1`. You are given a list `prerequisites` where `prerequisites[i] = [a, b]` means that course `b` must be completed before you can take course `a`.

Return `true` if it is possible to complete every course, and `false` otherwise.

<!-- examples -->

**Constraints:**

- `1 <= numCourses <= 2000`
- `0 <= prerequisites.length <= 5000`
- `prerequisites[i].length == 2`
- `0 <= a, b < numCourses`
- All pairs `[a, b]` are distinct.
