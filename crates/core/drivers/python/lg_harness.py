# Local Grind Python harness. Loads solution.py, decodes each test line from the input file,
# calls the solution and streams results back to the judge (see crates/core/src/lang/mod.rs).
import collections
import heapq
import io
import itertools
import json
import math
import re
import string
import sys
import threading
import time
import traceback
from bisect import bisect_left, bisect_right, insort
from collections import Counter, OrderedDict, defaultdict, deque
from functools import cache, cmp_to_key, lru_cache, reduce
from heapq import heapify, heappop, heappush, heappushpop, heapreplace, nlargest, nsmallest
from itertools import accumulate, combinations, permutations, product
from math import ceil, floor, gcd, inf, isqrt, log, log2, sqrt
from typing import Dict, List, Optional, Set, Tuple

SENTINEL = "\x1eLG "
MAX_NODES = 1_000_000


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


def parse_type(t):
    depth = 0
    while t.endswith("[]"):
        t = t[:-2]
        depth += 1
    return t, depth


def make_list(vals):
    dummy = cur = ListNode()
    for v in vals:
        cur.next = ListNode(v)
        cur = cur.next
    return dummy.next


def make_tree(vals):
    if not vals or vals[0] is None:
        return None
    root = TreeNode(vals[0])
    queue = deque([root])
    i = 1
    while queue and i < len(vals):
        node = queue.popleft()
        if i < len(vals) and vals[i] is not None:
            node.left = TreeNode(vals[i])
            queue.append(node.left)
        i += 1
        if i < len(vals) and vals[i] is not None:
            node.right = TreeNode(vals[i])
            queue.append(node.right)
        i += 1
    return root


def make_graph(adj):
    if not adj:
        return None
    nodes = [Node(i + 1) for i in range(len(adj))]
    for i, nbrs in enumerate(adj):
        nodes[i].neighbors = [nodes[j - 1] for j in nbrs]
    return nodes[0]


def list_out(head):
    out = []
    while head is not None:
        out.append(head.val)
        head = head.next
        if len(out) > MAX_NODES:
            raise RuntimeError("linked list output is too long (is there a cycle?)")
    return out


def tree_out(root):
    out, queue = [], deque([root])
    while queue:
        node = queue.popleft()
        if node is None:
            out.append(None)
            continue
        out.append(node.val)
        if len(out) > 2 * MAX_NODES:
            raise RuntimeError("tree output is too large (is there a cycle?)")
        queue.append(node.left)
        queue.append(node.right)
    while out and out[-1] is None:
        out.pop()
    return out


def graph_out(node):
    if node is None:
        return []
    seen, queue = {node.val: node}, deque([node])
    while queue:
        cur = queue.popleft()
        for n in cur.neighbors:
            if n.val not in seen:
                seen[n.val] = n
                queue.append(n)
    return [[n.val for n in seen[v].neighbors] for v in range(1, len(seen) + 1)]


def decode(value, t):
    base, depth = parse_type(t)
    if depth:
        inner = t[:-2]
        return None if value is None else [decode(v, inner) for v in value]
    if base == "ListNode":
        return make_list(value)
    if base == "TreeNode":
        return make_tree(value)
    if base == "Node":
        return make_graph(value)
    if base == "double":
        return float(value)
    return value


def encode(value, t):
    base, depth = parse_type(t)
    if depth:
        if value is None:
            return None
        inner = t[:-2]
        return [encode(v, inner) for v in value]
    if base == "ListNode":
        return list_out(value)
    if base == "TreeNode":
        return tree_out(value)
    if base == "Node":
        return graph_out(value)
    if base == "void":
        return None
    # JSON has no NaN/Infinity; emit null like the other language harnesses.
    if isinstance(value, float) and not math.isfinite(value):
        return None
    return value


def user_traceback():
    exc_type, exc, tb = sys.exc_info()
    frames = [f for f in traceback.extract_tb(tb) if f.filename == "solution.py"]
    lines = ["Traceback (most recent call last):\n"] + traceback.format_list(frames)
    lines += traceback.format_exception_only(exc_type, exc)
    return "".join(lines).rstrip()


class Emitter:
    def __init__(self):
        self.out = sys.__stdout__

    def emit(self, obj):
        sys.stdout.flush()
        self.out.write(SENTINEL + json.dumps(obj, separators=(",", ":")) + "\n")
        self.out.flush()


# What solution code can use without importing, mirroring a typical LeetCode Python 3
# environment: whole modules, plus star imports from modules whose names cannot shadow builtins
# (`math`, `operator` and `random` are deliberately not star-imported: they would replace the
# three-argument `pow` or the `random` module itself).
USER_PRELUDE = """
from bisect import *
from collections import *
from functools import *
from heapq import *
from itertools import *
from string import *
from typing import *
from math import (ceil, comb, factorial, floor, gcd, inf, isqrt, lcm, log, log2, log10, nan,
                  perm, prod, sqrt)
# Module imports come last so that e.g. `bisect` names the module, not bisect.bisect.
import bisect, collections, copy, datetime, functools, heapq, itertools, json, math, operator
import random, re, statistics, string, sys
"""


def load_solution():
    with open("solution.py", encoding="utf-8") as f:
        src = f.read()
    namespace = {k: v for k, v in globals().items() if not k.startswith("__")}
    exec(USER_PRELUDE, namespace)
    namespace["__name__"] = "solution"
    code = compile(src, "solution.py", "exec")
    exec(code, namespace)
    return namespace


def find_tree_node(root, val):
    queue = deque([root])
    while queue:
        node = queue.popleft()
        if node is None:
            continue
        if node.val == val:
            return node
        queue.append(node.left)
        queue.append(node.right)
    return None


def graph_nodes(node):
    seen, stack = set(), [node]
    while stack:
        cur = stack.pop()
        if cur is None or id(cur) in seen:
            continue
        seen.add(id(cur))
        stack.extend(getattr(cur, "neighbors", []) or [])
        for attr in ("next", "left", "right"):
            stack.append(getattr(cur, attr, None))
    return seen


def run_function(ns, sig, args):
    specs = sig["params"]
    names = [p["name"] for p in specs]
    decoded = [None if p.get("ref_of") else decode(a, p["type"]) for a, p in zip(args, specs)]
    for i, p in enumerate(specs):
        if p.get("ref_of"):
            decoded[i] = find_tree_node(decoded[names.index(p["ref_of"])], args[i])
    special = sig.get("special")
    hidden = [i for i, p in enumerate(specs) if p.get("hidden")]
    if special == "list_cycle":
        pos = args[hidden[0]]
        head = decoded[[p["type"] for p in specs].index("ListNode")]
        if head is not None and pos >= 0:
            nodes = []
            while head is not None:
                nodes.append(head)
                head = head.next
            nodes[-1].next = nodes[pos]
    elif special == "bad_version":
        bad = args[hidden[0]]
        ns["isBadVersion"] = lambda version: version >= bad
    before = graph_nodes(decoded[0]) if special == "deep_copy" and decoded else None
    call_args = [v for i, v in enumerate(decoded) if i not in hidden]
    method = getattr(ns["Solution"](), sig["name"])
    start = time.perf_counter_ns()
    result = method(*call_args)
    elapsed = time.perf_counter_ns() - start
    if before is not None and before & graph_nodes(result):
        raise RuntimeError("the returned structure shares nodes with the input; a deep copy is required")
    out_param = sig.get("output_param")
    if out_param:
        idx = names.index(out_param)
        return encode(decoded[idx], specs[idx]["type"]), elapsed
    if sig.get("output_mode") == "node_val":
        return (None if result is None else result.val), elapsed
    return encode(result, sig["returns"]), elapsed


def run_codec(ns, sig, args):
    enc, dec = sig["encode"], sig["decode"]
    value = decode(args[0], enc["params"][0]["type"])
    cls = ns[sig["class"]]
    start = time.perf_counter_ns()
    data = getattr(cls(), enc["name"])(value)
    middle = time.perf_counter_ns()
    if enc["returns"] == "string" and not isinstance(data, str):
        raise TypeError("%s must return a str, got %s" % (enc["name"], type(data).__name__))
    resume = time.perf_counter_ns()
    result = getattr(cls(), dec["name"])(data)
    elapsed = (middle - start) + (time.perf_counter_ns() - resume)
    return encode(result, dec["returns"]), elapsed


def run_design(ns, sig, args):
    ops, op_args = args
    methods = {m["name"]: m for m in sig["methods"]}
    ctor_types = [p["type"] for p in sig.get("constructor", [])]
    out, elapsed, obj = [], 0, None
    for op, a in zip(ops, op_args):
        if obj is None:
            decoded = [decode(v, t) for v, t in zip(a, ctor_types)]
            start = time.perf_counter_ns()
            obj = ns[sig["class"]](*decoded)
            elapsed += time.perf_counter_ns() - start
            out.append(None)
            continue
        m = methods[op]
        decoded = [decode(v, p["type"]) for v, p in zip(a, m.get("params", []))]
        start = time.perf_counter_ns()
        res = getattr(obj, op)(*decoded)
        elapsed += time.perf_counter_ns() - start
        out.append(encode(res, m["returns"]))
    return out, elapsed


def main():
    emitter = Emitter()
    with open("signature.json", encoding="utf-8") as f:
        sig = json.load(f)
    try:
        ns = load_solution()
    except SyntaxError as e:
        msg = "Line %s: %s: %s" % (e.lineno, type(e).__name__, e.msg)
        emitter.emit({"compile_error": msg})
        return
    except BaseException:
        emitter.emit({"i": 0, "error": user_traceback()})
        return
    run = {"design": run_design, "codec": run_codec}.get(sig["kind"], run_function)
    with open(sys.argv[1], encoding="utf-8") as f:
        for i, line in enumerate(f):
            line = line.strip()
            if not line:
                continue
            try:
                out, elapsed = run(ns, sig, json.loads(line))
                payload = json.dumps(out, separators=(",", ":"))
            except BaseException:
                emitter.emit({"i": i, "error": user_traceback()})
                return
            sys.stdout.flush()
            emitter.out.write("%s{\"i\":%d,\"ns\":%d,\"out\":%s}\n" % (SENTINEL, i, elapsed, payload))
            emitter.out.flush()


if __name__ == "__main__":
    sys.setrecursionlimit(200000)
    threading.stack_size(512 * 1024 * 1024)
    t = threading.Thread(target=main)
    t.start()
    t.join()
