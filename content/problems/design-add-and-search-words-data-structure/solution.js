var WordDictionary = function() {
    this.root = {};
};

/**
 * @param {string} word
 * @return {void}
 */
WordDictionary.prototype.addWord = function(word) {
    let node = this.root;
    for (const ch of word) node = node[ch] ??= {};
    node.$ = true;
};

/**
 * @param {string} word
 * @return {boolean}
 */
WordDictionary.prototype.search = function(word) {
    let level = [this.root];
    for (const ch of word) {
        const next = [];
        if (ch === '.') {
            for (const node of level)
                for (const key in node)
                    if (key !== '$') next.push(node[key]);
        } else {
            for (const node of level)
                if (node[ch] !== undefined) next.push(node[ch]);
        }
        if (!next.length) return false;
        level = next;
    }
    return level.some(node => node.$ === true);
};
