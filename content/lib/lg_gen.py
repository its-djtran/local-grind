"""Shared helpers for Local Grind test generators.

A generator prints one JSON array of arguments per line. Use `emit(*args)` for each test and
seed all randomness through `rng(seed)` so builds are reproducible.
"""
import json
import random
import sys


def rng(seed=0):
    return random.Random(seed)


def emit(*args):
    """Writes one test case (the positional arguments of the solution call)."""
    sys.stdout.write(json.dumps(list(args), separators=(",", ":")) + "\n")


def int_array(r, n, lo, hi):
    return [r.randint(lo, hi) for _ in range(n)]


def distinct_ints(r, n, lo, hi):
    return r.sample(range(lo, hi + 1), n)


def random_string(r, n, alphabet="abcdefghijklmnopqrstuvwxyz"):
    return "".join(r.choice(alphabet) for _ in range(n))


def tree_level_order(values_in_bfs, r=None, null_prob=0.0):
    """Builds a LeetCode level-order array from BFS values, optionally dropping subtrees."""
    if not values_in_bfs:
        return []
    out = [values_in_bfs[0]]
    frontier = 1
    i = 1
    while i < len(values_in_bfs) and frontier:
        next_frontier = 0
        for _ in range(frontier * 2):
            if i >= len(values_in_bfs):
                break
            if r is not None and r.random() < null_prob:
                out.append(None)
            else:
                out.append(values_in_bfs[i])
                i += 1
                next_frontier += 1
        frontier = next_frontier
    while out and out[-1] is None:
        out.pop()
    return out


def bst_level_order(sorted_values):
    """Level-order array of a height-balanced BST over sorted distinct values."""
    if not sorted_values:
        return []
    nodes = {}

    def build(lo, hi, idx):
        if lo > hi:
            return
        mid = (lo + hi) // 2
        nodes[idx] = sorted_values[mid]
        build(lo, mid - 1, 2 * idx + 1)
        build(mid + 1, hi, 2 * idx + 2)

    build(0, len(sorted_values) - 1, 0)
    # Convert heap-indexed nodes into LeetCode level order (children of null are omitted).
    out, queue = [], [0]
    while queue:
        nxt = []
        for idx in queue:
            if idx in nodes:
                out.append(nodes[idx])
                nxt += [2 * idx + 1, 2 * idx + 2]
            else:
                out.append(None)
        queue = nxt
    while out and out[-1] is None:
        out.pop()
    return out
