// Local Grind JavaScript harness. Loads solution.js, decodes each test line from the input
// file, calls the solution and streams results back to the judge (see crates/core/src/lang/mod.rs).
// jsc exposes script arguments as a global `arguments`; capture it before entering the harness
// scope (inside a function, `arguments` means something else).
const __lgScriptArgs = typeof process === "undefined" && typeof arguments !== "undefined" ? Array.from(arguments) : [];

// Everything lives in a function scope so harness helpers (decode, encode, ...) can never be
// replaced by same-named globals in user code: jsc runs this file as a plain script.
(function (scriptArgs) {
"use strict";

const SENTINEL = "\x1eLG ";
const MAX_NODES = 1000000;
const IS_NODE = typeof process !== "undefined" && typeof require === "function";

// Runtime layer: Node.js, or JavaScriptCore's `jsc` shell (ships with macOS) when Node is not
// installed. Everything below the runtime layer is shared.
let RT = null;

function nodeRuntime(fs, util, vm) {
  // Pipes may be non-blocking, so writes loop over partial writes and retry on EAGAIN.
  const pause = new Int32Array(new SharedArrayBuffer(4));
  const write = (fd, text) => {
    const buf = Buffer.from(text);
    let off = 0;
    while (off < buf.length) {
      try {
        off += fs.writeSync(fd, buf, off, buf.length - off);
      } catch (e) {
        if (e.code !== "EAGAIN") throw e;
        Atomics.wait(pause, 0, 0, 1);
      }
    }
  };
  const rt = {
    read: (path) => fs.readFileSync(path, "utf8"),
    out: (text) => write(1, text),
    err: (text) => write(2, text),
    format: (...a) => util.format(...a),
    inspect: (v) => util.inspect(v),
    now: () => performance.now() * 1e6,
    inputPath: () => process.argv[2],
    load: (src) => {
      let script;
      try {
        script = new vm.Script(src, { filename: "solution.js" });
      } catch (e) {
        return { syntax: `${String(e.stack || "").split("\n")[0]}\n${e.name}: ${e.message}` };
      }
      script.runInThisContext();
      return {};
    },
    lookup: (name) => vm.runInThisContext(name),
  };
  process.stdout.write = (chunk) => (rt.out(String(chunk)), true);
  return rt;
}

if (IS_NODE) {
  const fs = require("fs");
  const util = require("util");
  const vm = require("vm");
  const { Worker, isMainThread } = require("worker_threads");
  if (isMainThread) {
    // Run in a worker so deep recursion gets a large stack, like LeetCode's judge.
    const worker = new Worker(__filename, {
      argv: process.argv.slice(2),
      resourceLimits: { stackSizeMb: 256 },
      stdout: false,
      stderr: false,
    });
    worker.on("error", (e) => {
      fs.writeSync(2, String((e && e.stack) || e) + "\n");
      process.exitCode = 1;
    });
    worker.on("exit", (code) => {
      if (code !== 0 && !process.exitCode) process.exitCode = code;
    });
  } else {
    RT = nodeRuntime(fs, util, vm);
  }
}
if (!IS_NODE) {
  const show = (v) => (typeof v === "string" ? v : RT.inspect(v));
  RT = {
    read: (path) => readFile(path),
    out: (text) => {
      // print() appends a newline; strip one so callers control line endings.
      print(text.endsWith("\n") ? text.slice(0, -1) : text);
    },
    err: (text) => printErr(text.endsWith("\n") ? text.slice(0, -1) : text),
    format: (...a) => a.map(show).join(" "),
    inspect: (v) => {
      try {
        return typeof v === "string" ? JSON.stringify(v) : JSON.stringify(v) ?? String(v);
      } catch {
        return String(v);
      }
    },
    now: () => preciseTime() * 1e9,
    inputPath: () => scriptArgs[0],
    load: (src) => {
      try {
        checkSyntax("solution.js");
      } catch (e) {
        return { syntax: `solution.js:${e.line ?? "?"}\n${e.name}: ${e.message}` };
      }
      load("solution.js");
      return {};
    },
    lookup: (name) => (0, eval)(name),
  };
}

// Synchronous console so user output and result lines stay in order.
if (RT) {
  if (typeof console === "undefined") globalThis.console = {};
  console.log = console.info = console.debug = (...a) => RT.out(RT.format(...a) + "\n");
  console.error = console.warn = (...a) => RT.err(RT.format(...a) + "\n");
}

function ListNode(val, next) {
  this.val = val === undefined ? 0 : val;
  this.next = next === undefined ? null : next;
}

function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

function _Node(val, neighbors) {
  this.val = val === undefined ? 0 : val;
  this.neighbors = neighbors === undefined ? [] : neighbors;
}

Object.assign(globalThis, { ListNode, TreeNode, _Node, Node: _Node });

function parseType(t) {
  let depth = 0;
  while (t.endsWith("[]")) {
    t = t.slice(0, -2);
    depth++;
  }
  return [t, depth];
}

function makeList(vals) {
  const dummy = new ListNode();
  let cur = dummy;
  for (const v of vals) cur = cur.next = new ListNode(v);
  return dummy.next;
}

function makeTree(vals) {
  if (!vals.length || vals[0] === null) return null;
  const root = new TreeNode(vals[0]);
  const queue = [root];
  let head = 0;
  let i = 1;
  while (head < queue.length && i < vals.length) {
    const node = queue[head++];
    if (i < vals.length && vals[i] !== null) queue.push((node.left = new TreeNode(vals[i])));
    i++;
    if (i < vals.length && vals[i] !== null) queue.push((node.right = new TreeNode(vals[i])));
    i++;
  }
  return root;
}

function makeGraph(adj) {
  if (!adj.length) return null;
  const nodes = adj.map((_, i) => new _Node(i + 1));
  adj.forEach((nbrs, i) => (nodes[i].neighbors = nbrs.map((j) => nodes[j - 1])));
  return nodes[0];
}

function listOut(head) {
  const out = [];
  while (head) {
    out.push(head.val);
    head = head.next;
    if (out.length > MAX_NODES) throw new Error("linked list output is too long (is there a cycle?)");
  }
  return out;
}

function treeOut(root) {
  const out = [];
  const queue = [root];
  let head = 0;
  while (head < queue.length) {
    const node = queue[head++];
    if (!node) {
      out.push(null);
      continue;
    }
    out.push(node.val);
    if (out.length > 2 * MAX_NODES) throw new Error("tree output is too large (is there a cycle?)");
    queue.push(node.left ?? null, node.right ?? null);
  }
  while (out.length && out[out.length - 1] === null) out.pop();
  return out;
}

function graphOut(node) {
  if (!node) return [];
  const seen = new Map([[node.val, node]]);
  const queue = [node];
  for (let h = 0; h < queue.length; h++) {
    for (const n of queue[h].neighbors) {
      if (!seen.has(n.val)) {
        seen.set(n.val, n);
        queue.push(n);
      }
    }
  }
  const out = [];
  for (let v = 1; v <= seen.size; v++) out.push(seen.get(v).neighbors.map((n) => n.val));
  return out;
}

function decode(value, t) {
  const [base, depth] = parseType(t);
  if (depth) return value === null ? null : value.map((v) => decode(v, t.slice(0, -2)));
  if (base === "ListNode") return makeList(value);
  if (base === "TreeNode") return makeTree(value);
  if (base === "Node") return makeGraph(value);
  return value;
}

function encode(value, t) {
  const [base, depth] = parseType(t);
  if (value === undefined) return null;
  if (depth) return value === null ? null : Array.from(value, (v) => encode(v, t.slice(0, -2)));
  if (base === "ListNode") return listOut(value);
  if (base === "TreeNode") return treeOut(value);
  if (base === "Node") return graphOut(value);
  if (base === "void") return null;
  if (typeof value === "bigint") return Number(value);
  return value;
}

function findTreeNode(root, val) {
  const queue = [root];
  for (let h = 0; h < queue.length; h++) {
    const node = queue[h];
    if (!node) continue;
    if (node.val === val) return node;
    queue.push(node.left, node.right);
  }
  return null;
}

function reachable(node) {
  const seen = new Set();
  const stack = [node];
  while (stack.length) {
    const cur = stack.pop();
    if (!cur || typeof cur !== "object" || seen.has(cur)) continue;
    seen.add(cur);
    for (const k of ["next", "left", "right"]) stack.push(cur[k]);
    if (Array.isArray(cur.neighbors)) stack.push(...cur.neighbors);
  }
  return seen;
}

function userStack(e) {
  if (!(e instanceof Error)) return "Uncaught " + RT.inspect(e);
  const frames = String(e.stack || "")
    .split("\n")
    .filter((l) => l.includes("solution.js"))
    .map((l) => l.replace(/[^\s@(]*\/solution\.js/, "solution.js"));
  return [`${e.name}: ${e.message}`].concat(frames).join("\n");
}

function lookup(name) {
  try {
    return RT.lookup(name);
  } catch {
    throw new Error(`${name} is not defined`);
  }
}

const now = () => RT.now();

function runFunction(sig, args) {
  const specs = sig.params;
  const names = specs.map((p) => p.name);
  const decoded = specs.map((p, i) => (p.ref_of ? null : decode(args[i], p.type)));
  specs.forEach((p, i) => {
    if (p.ref_of) decoded[i] = findTreeNode(decoded[names.indexOf(p.ref_of)], args[i]);
  });
  const hidden = specs.map((p, i) => (p.hidden ? i : -1)).filter((i) => i >= 0);
  let fn;
  if (sig.special === "list_cycle") {
    const pos = args[hidden[0]];
    let head = decoded[specs.findIndex((p) => p.type === "ListNode")];
    if (head && pos >= 0) {
      const nodes = [];
      for (; head; head = head.next) nodes.push(head);
      nodes[nodes.length - 1].next = nodes[pos];
    }
  }
  if (sig.special === "bad_version") {
    const bad = args[hidden[0]];
    const isBadVersion = (version) => version >= bad;
    globalThis.isBadVersion = isBadVersion;
    fn = lookup("solution")(isBadVersion);
  } else {
    fn = lookup(sig.name);
  }
  const before = sig.special === "deep_copy" ? reachable(decoded[0]) : null;
  const callArgs = decoded.filter((_, i) => !hidden.includes(i));
  const start = now();
  const result = fn(...callArgs);
  const elapsed = now() - start;
  if (before) {
    for (const n of reachable(result)) {
      if (before.has(n)) throw new Error("the returned structure shares nodes with the input; a deep copy is required");
    }
  }
  if (sig.output_param) {
    const idx = names.indexOf(sig.output_param);
    return [encode(decoded[idx], specs[idx].type), elapsed];
  }
  if (sig.output_mode === "node_val") return [result ? result.val : null, elapsed];
  return [encode(result, sig.returns), elapsed];
}

function runDesign(sig, args) {
  const [ops, opArgs] = args;
  const methods = new Map(sig.methods.map((m) => [m.name, m]));
  const ctorTypes = (Array.isArray(sig.constructor) ? sig.constructor : []).map((p) => p.type);
  const Cls = lookup(sig.class);
  const out = [];
  let elapsed = 0;
  let obj = null;
  ops.forEach((op, k) => {
    if (k === 0) {
      const decoded = opArgs[0].map((v, i) => decode(v, ctorTypes[i]));
      const start = now();
      obj = new Cls(...decoded);
      elapsed += now() - start;
      out.push(null);
      return;
    }
    const m = methods.get(op);
    const decoded = opArgs[k].map((v, i) => decode(v, m.params[i].type));
    const start = now();
    const res = obj[op](...decoded);
    elapsed += now() - start;
    out.push(encode(res, m.returns));
  });
  return [out, elapsed];
}

function runCodec(sig, args) {
  const enc = lookup(sig.encode.name);
  const dec = lookup(sig.decode.name);
  const value = decode(args[0], sig.encode.params[0].type);
  const start = now();
  const data = enc(value);
  const middle = now();
  if (sig.encode.returns === "string" && typeof data !== "string") {
    throw new TypeError(`${sig.encode.name} must return a string, got ${typeof data}`);
  }
  const resume = now();
  const result = dec(data);
  const elapsed = middle - start + (now() - resume);
  return [encode(result, sig.decode.returns), elapsed];
}

function emit(obj) {
  RT.out(SENTINEL + JSON.stringify(obj) + "\n");
}

function main() {
  const sig = JSON.parse(RT.read("signature.json"));
  try {
    const loaded = RT.load(RT.read("solution.js"));
    if (loaded.syntax) {
      emit({ compile_error: loaded.syntax });
      return;
    }
  } catch (e) {
    emit({ i: 0, error: userStack(e) });
    return;
  }
  const run = sig.kind === "design" ? runDesign : sig.kind === "codec" ? runCodec : runFunction;
  const lines = RT.read(RT.inputPath()).split("\n");
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line) continue;
    let out;
    let elapsed;
    try {
      [out, elapsed] = run(sig, JSON.parse(line));
      out = JSON.stringify(out === undefined ? null : out);
    } catch (e) {
      emit({ i, error: userStack(e) });
      return;
    }
    RT.out(`${SENTINEL}{"i":${i},"ns":${Math.max(0, Math.round(elapsed))},"out":${out}}\n`);
  }
}

// In Node's main thread RT stays null: the worker started above does the judging.
if (RT) main();
})(__lgScriptArgs);
