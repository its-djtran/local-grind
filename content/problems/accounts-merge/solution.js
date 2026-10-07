/**
 * @param {string[][]} accounts
 * @return {string[][]}
 */
var accountsMerge = function(accounts) {
    const parent = new Map();
    const owner = new Map();
    const find = (x) => {
        while (parent.get(x) !== x) {
            const gp = parent.get(parent.get(x));
            parent.set(x, gp);
            x = gp;
        }
        return x;
    };
    for (const [name, ...emails] of accounts) {
        for (const e of emails) {
            if (!parent.has(e)) {
                parent.set(e, e);
                owner.set(e, name);
            }
        }
        const root = find(emails[0]);
        for (let i = 1; i < emails.length; i++) {
            const other = find(emails[i]);
            if (other !== root) parent.set(other, root);
        }
    }
    const groups = new Map();
    for (const e of parent.keys()) {
        const r = find(e);
        if (!groups.has(r)) groups.set(r, []);
        groups.get(r).push(e);
    }
    const res = [];
    for (const [root, emails] of groups) {
        emails.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
        res.push([owner.get(root), ...emails]);
    }
    return res;
};
