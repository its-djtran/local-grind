// Local Grind JavaScript harness. Loads solution.js, decodes each test line from the input
// file, calls the solution and streams results back to the judge (see crates/core/src/lang/mod.rs).
"use strict";
const fs = require("fs");
const util = require("util");
const vm = require("vm");
const { Worker, isMainThread } = require("worker_threads");

const SENTINEL = "\x1eLG ";
const MAX_NODES = 1000000;

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
  return;
}

// Synchronous console so user output and result lines stay in order. Pipes may be
// non-blocking, so writes loop over partial writes and retry on EAGAIN.
const pause = new Int32Array(new SharedArrayBuffer(4));
function write(fd, s) {
  const buf = Buffer.from(s);
  let off = 0;
  while (off < buf.length) {
    try {
      off += fs.writeSync(fd, buf, off, buf.length - off);
    } catch (e) {
      if (e.code !== "EAGAIN") throw e;
      Atomics.wait(pause, 0, 0, 1);
    }
  }
}
console.log = console.info = console.debug = (...a) => write(1, util.format(...a) + "\n");
console.error = console.warn = (...a) => write(2, util.format(...a) + "\n");
process.stdout.write = (chunk) => (write(1, String(chunk)), true);

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
  if (!(e instanceof Error)) return "Uncaught " + util.inspect(e);
  const lines = String(e.stack || e).split("\n");
  const keep = [lines[0]].concat(lines.slice(1).filter((l) => l.includes("solution.js")));
  return keep.join("\n");
}

function lookup(name) {
  try {
    return vm.runInThisContext(name);
  } catch {
    throw new Error(`${name} is not defined`);
  }
}

const now = () => process.hrtime.bigint();

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
  let elapsed = 0n;
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
  write(1, SENTINEL + JSON.stringify(obj) + "\n");
}

function main() {
  const sig = JSON.parse(fs.readFileSync("signature.json", "utf8"));
  const src = fs.readFileSync("solution.js", "utf8");
  let script;
  try {
    script = new vm.Script(src, { filename: "solution.js" });
  } catch (e) {
    const where = String(e.stack || "").split("\n")[0];
    emit({ compile_error: `${where}\n${e.name}: ${e.message}` });
    return;
  }
  try {
    script.runInThisContext();
  } catch (e) {
    emit({ i: 0, error: userStack(e) });
    return;
  }
  const run = sig.kind === "design" ? runDesign : sig.kind === "codec" ? runCodec : runFunction;
  const lines = fs.readFileSync(process.argv[2], "utf8").split("\n");
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
    write(1, `${SENTINEL}{"i":${i},"ns":${elapsed},"out":${out}}\n`);
  }
}

main();
