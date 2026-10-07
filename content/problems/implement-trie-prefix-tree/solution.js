var Trie = function() {
    this.root = {};
};

/**
 * @param {string} word
 * @return {void}
 */
Trie.prototype.insert = function(word) {
    let node = this.root;
    for (const ch of word) node = node[ch] ??= {};
    node.$ = true;
};

Trie.prototype._walk = function(s) {
    let node = this.root;
    for (const ch of s) {
        node = node[ch];
        if (node === undefined) return null;
    }
    return node;
};

/**
 * @param {string} word
 * @return {boolean}
 */
Trie.prototype.search = function(word) {
    const node = this._walk(word);
    return node !== null && node.$ === true;
};

/**
 * @param {string} prefix
 * @return {boolean}
 */
Trie.prototype.startsWith = function(prefix) {
    return this._walk(prefix) !== null;
};
