// Local Grind C++ driver: JSON decoding/encoding of judge values and the test loop. The
// generated lg_case() (see crates/core/src/lang/cpp.rs) decodes one test and calls the solution.
#pragma once
#include <chrono>
#include <exception>
#include <fstream>
#include <pthread.h>

namespace lg {

struct J {
    enum Kind { Null, Bool, Num, Str, Arr } kind = Null;
    bool b = false;
    std::string text;  // number literal or string contents
    std::vector<J> items;
};

struct Parser {
    const std::string& s;
    size_t i = 0;

    void ws() {
        while (i < s.size() && (s[i] == ' ' || s[i] == '\t' || s[i] == '\r' || s[i] == '\n')) i++;
    }

    static void put_utf8(std::string& out, unsigned cp) {
        if (cp < 0x80) {
            out += char(cp);
        } else if (cp < 0x800) {
            out += char(0xC0 | (cp >> 6));
            out += char(0x80 | (cp & 0x3F));
        } else if (cp < 0x10000) {
            out += char(0xE0 | (cp >> 12));
            out += char(0x80 | ((cp >> 6) & 0x3F));
            out += char(0x80 | (cp & 0x3F));
        } else {
            out += char(0xF0 | (cp >> 18));
            out += char(0x80 | ((cp >> 12) & 0x3F));
            out += char(0x80 | ((cp >> 6) & 0x3F));
            out += char(0x80 | (cp & 0x3F));
        }
    }

    J value() {
        ws();
        J j;
        if (i >= s.size()) throw std::runtime_error("unexpected end of input");
        char c = s[i];
        if (c == '[') {
            j.kind = J::Arr;
            i++;
            ws();
            if (s[i] == ']') { i++; return j; }
            while (true) {
                j.items.push_back(value());
                ws();
                if (s[i] == ',') { i++; continue; }
                if (s[i] == ']') { i++; break; }
                throw std::runtime_error("bad array");
            }
        } else if (c == '"') {
            j.kind = J::Str;
            i++;
            while (s[i] != '"') {
                if (s[i] == '\\') {
                    char e = s[++i];
                    switch (e) {
                        case 'n': j.text += '\n'; break;
                        case 't': j.text += '\t'; break;
                        case 'r': j.text += '\r'; break;
                        case 'b': j.text += '\b'; break;
                        case 'f': j.text += '\f'; break;
                        case 'u': {
                            unsigned cp = std::stoul(s.substr(i + 1, 4), nullptr, 16);
                            i += 4;
                            if (cp >= 0xD800 && cp < 0xDC00 && s[i + 1] == '\\' && s[i + 2] == 'u') {
                                unsigned lo = std::stoul(s.substr(i + 3, 4), nullptr, 16);
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                                i += 6;
                            }
                            put_utf8(j.text, cp);
                            break;
                        }
                        default: j.text += e;
                    }
                    i++;
                } else {
                    j.text += s[i++];
                }
            }
            i++;
        } else if (s.compare(i, 4, "null") == 0) {
            i += 4;
        } else if (s.compare(i, 4, "true") == 0) {
            j.kind = J::Bool;
            j.b = true;
            i += 4;
        } else if (s.compare(i, 5, "false") == 0) {
            j.kind = J::Bool;
            i += 5;
        } else {
            j.kind = J::Num;
            size_t start = i;
            while (i < s.size() && (isdigit((unsigned char)s[i]) || s[i] == '-' || s[i] == '+' || s[i] == '.' || s[i] == 'e' || s[i] == 'E')) i++;
            j.text = s.substr(start, i - start);
        }
        return j;
    }
};

inline J parse(const std::string& s) {
    Parser p{s};
    return p.value();
}

template <class T> struct Dec;
template <> struct Dec<int> { static int get(const J& j) { return (int)std::stoll(j.text); } };
template <> struct Dec<long long> { static long long get(const J& j) { return std::stoll(j.text); } };
template <> struct Dec<double> { static double get(const J& j) { return std::stod(j.text); } };
template <> struct Dec<bool> { static bool get(const J& j) { return j.b; } };
template <> struct Dec<std::string> { static std::string get(const J& j) { return j.text; } };
template <> struct Dec<char> { static char get(const J& j) { return j.text.empty() ? '\0' : j.text[0]; } };
template <class T> struct Dec<std::vector<T>> {
    static std::vector<T> get(const J& j) {
        std::vector<T> out;
        out.reserve(j.items.size());
        for (const J& x : j.items) out.push_back(Dec<T>::get(x));
        return out;
    }
};
template <> struct Dec<ListNode*> {
    static ListNode* get(const J& j) {
        ListNode dummy;
        ListNode* cur = &dummy;
        for (const J& x : j.items) cur = cur->next = new ListNode(Dec<int>::get(x));
        return dummy.next;
    }
};
template <> struct Dec<TreeNode*> {
    static TreeNode* get(const J& j) {
        const auto& v = j.items;
        if (v.empty() || v[0].kind == J::Null) return nullptr;
        TreeNode* root = new TreeNode(Dec<int>::get(v[0]));
        std::deque<TreeNode*> q{root};
        size_t i = 1;
        while (!q.empty() && i < v.size()) {
            TreeNode* n = q.front();
            q.pop_front();
            if (i < v.size() && v[i].kind != J::Null) q.push_back(n->left = new TreeNode(Dec<int>::get(v[i])));
            i++;
            if (i < v.size() && v[i].kind != J::Null) q.push_back(n->right = new TreeNode(Dec<int>::get(v[i])));
            i++;
        }
        return root;
    }
};
template <> struct Dec<Node*> {
    static Node* get(const J& j) {
        if (j.items.empty()) return nullptr;
        std::vector<Node*> nodes;
        for (size_t k = 0; k < j.items.size(); k++) nodes.push_back(new Node((int)k + 1));
        for (size_t k = 0; k < j.items.size(); k++)
            for (const J& x : j.items[k].items) nodes[k]->neighbors.push_back(nodes[Dec<int>::get(x) - 1]);
        return nodes[0];
    }
};

const size_t MAX_NODES = 1000000;

inline void enc(std::string& o, int v) { o += std::to_string(v); }
inline void enc(std::string& o, long long v) { o += std::to_string(v); }
inline void enc(std::string& o, long v) { o += std::to_string(v); }
inline void enc(std::string& o, unsigned v) { o += std::to_string(v); }
inline void enc(std::string& o, unsigned long v) { o += std::to_string(v); }
inline void enc(std::string& o, unsigned long long v) { o += std::to_string(v); }
inline void enc(std::string& o, bool v) { o += v ? "true" : "false"; }
inline void enc(std::string& o, double v) {
    if (!std::isfinite(v)) { o += "null"; return; }
    char buf[32];
    std::snprintf(buf, sizeof buf, "%.17g", v);
    o += buf;
}
inline void enc_str(std::string& o, const std::string& s) {
    o += '"';
    for (unsigned char c : s) {
        switch (c) {
            case '"': o += "\\\""; break;
            case '\\': o += "\\\\"; break;
            case '\n': o += "\\n"; break;
            case '\t': o += "\\t"; break;
            case '\r': o += "\\r"; break;
            default:
                if (c < 0x20) {
                    char buf[8];
                    std::snprintf(buf, sizeof buf, "\\u%04x", c);
                    o += buf;
                } else {
                    o += char(c);
                }
        }
    }
    o += '"';
}
inline void enc(std::string& o, const std::string& s) { enc_str(o, s); }
inline void enc(std::string& o, char c) { enc_str(o, std::string(1, c)); }
inline void enc(std::string& o, ListNode* h) {
    o += '[';
    size_t n = 0;
    for (; h; h = h->next) {
        if (n++) o += ',';
        if (n > MAX_NODES) throw std::runtime_error("linked list output is too long (is there a cycle?)");
        o += std::to_string(h->val);
    }
    o += ']';
}
inline void enc(std::string& o, TreeNode* root) {
    std::vector<TreeNode*> order;
    std::deque<TreeNode*> q{root};
    while (!q.empty()) {
        TreeNode* n = q.front();
        q.pop_front();
        order.push_back(n);
        if (order.size() > 2 * MAX_NODES) throw std::runtime_error("tree output is too large (is there a cycle?)");
        if (n) {
            q.push_back(n->left);
            q.push_back(n->right);
        }
    }
    while (!order.empty() && !order.back()) order.pop_back();
    o += '[';
    for (size_t k = 0; k < order.size(); k++) {
        if (k) o += ',';
        o += order[k] ? std::to_string(order[k]->val) : "null";
    }
    o += ']';
}
inline void enc(std::string& o, Node* node) {
    std::map<int, Node*> seen;
    if (node) {
        std::deque<Node*> q{node};
        seen[node->val] = node;
        while (!q.empty()) {
            Node* cur = q.front();
            q.pop_front();
            for (Node* n : cur->neighbors)
                if (n && !seen.count(n->val)) {
                    seen[n->val] = n;
                    q.push_back(n);
                }
        }
    }
    o += '[';
    for (int v = 1; v <= (int)seen.size(); v++) {
        if (v > 1) o += ',';
        o += '[';
        auto& nb = seen[v]->neighbors;
        for (size_t k = 0; k < nb.size(); k++) {
            if (k) o += ',';
            o += std::to_string(nb[k]->val);
        }
        o += ']';
    }
    o += ']';
}
template <class T> void enc(std::string& o, const std::vector<T>& v) {
    o += '[';
    for (size_t k = 0; k < v.size(); k++) {
        if (k) o += ',';
        if constexpr (std::is_same_v<T, bool>) enc(o, (bool)v[k]);  // vector<bool> proxies
        else enc(o, v[k]);
    }
    o += ']';
}

inline TreeNode* find(TreeNode* root, const J& value) {
    if (value.kind == J::Null) return nullptr;
    int target = Dec<int>::get(value);
    std::deque<TreeNode*> q{root};
    while (!q.empty()) {
        TreeNode* n = q.front();
        q.pop_front();
        if (!n) continue;
        if (n->val == target) return n;
        q.push_back(n->left);
        q.push_back(n->right);
    }
    return nullptr;
}

inline void link_cycle(ListNode* head, int pos) {
    if (!head || pos < 0) return;
    std::vector<ListNode*> nodes;
    for (; head; head = head->next) nodes.push_back(head);
    nodes.back()->next = nodes[pos];
}

template <class P> std::set<const void*> reachable(P root) {
    std::set<const void*> seen;
    std::vector<P> stack{root};
    while (!stack.empty()) {
        P cur = stack.back();
        stack.pop_back();
        if (!cur || !seen.insert(cur).second) continue;
        if constexpr (std::is_same_v<P, Node*>) {
            for (Node* n : cur->neighbors) stack.push_back(n);
        } else if constexpr (std::is_same_v<P, ListNode*>) {
            stack.push_back(cur->next);
        } else {
            stack.push_back(cur->left);
            stack.push_back(cur->right);
        }
    }
    return seen;
}

template <class P> void check_deep_copy(P input, P output) {
    auto before = reachable(input);
    for (const void* n : reachable(output))
        if (before.count(n)) throw std::runtime_error("the returned structure shares nodes with the input; a deep copy is required");
}

using clock = std::chrono::steady_clock;
inline long long ns(clock::time_point a, clock::time_point b) {
    return std::chrono::duration_cast<std::chrono::nanoseconds>(b - a).count();
}

}  // namespace lg

// Implemented by the generated lg_case.inc: decodes one test, runs it, appends the JSON output,
// and returns the solution time in nanoseconds.
long long lg_case(const lg::J& args, std::string& out);

static const char* lg_input_path;

static void lg_emit(const std::string& line) {
    std::cout.flush();
    std::fflush(stdout);
    std::fwrite(line.data(), 1, line.size(), stdout);
    std::fflush(stdout);
}

static std::string lg_json_string(const std::string& s) {
    std::string o;
    lg::enc_str(o, s);
    return o;
}

static void* lg_run(void*) {
    std::ifstream in(lg_input_path);
    std::string line;
    for (int i = 0; std::getline(in, line); i++) {
        if (line.empty()) continue;
        std::string out;
        long long elapsed = 0;
        try {
            elapsed = lg_case(lg::parse(line), out);
        } catch (const std::exception& e) {
            lg_emit("\x1eLG {\"i\":" + std::to_string(i) + ",\"error\":" + lg_json_string(std::string("terminate called after throwing an exception\n  what():  ") + e.what()) + "}\n");
            return nullptr;
        } catch (...) {
            lg_emit("\x1eLG {\"i\":" + std::to_string(i) + ",\"error\":\"terminate called after throwing an unknown exception\"}\n");
            return nullptr;
        }
        lg_emit("\x1eLG {\"i\":" + std::to_string(i) + ",\"ns\":" + std::to_string(elapsed) + ",\"out\":" + out + "}\n");
    }
    return nullptr;
}

int main(int argc, char** argv) {
    if (argc < 2) return 2;
    lg_input_path = argv[1];
    // A large stack lets recursive solutions handle deep (degenerate) trees and lists.
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setstacksize(&attr, size_t(512) << 20);
    pthread_t thread;
    if (pthread_create(&thread, &attr, lg_run, nullptr) != 0) lg_run(nullptr);
    else pthread_join(thread, nullptr);
    return 0;
}
