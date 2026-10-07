class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word: str) -> bool:
        level = [self.root]
        for ch in word:
            if ch == ".":
                level = [child for node in level for key, child in node.items() if key != "$"]
            else:
                level = [node[ch] for node in level if ch in node]
            if not level:
                return False
        return any("$" in node for node in level)
