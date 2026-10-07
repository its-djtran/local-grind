class Solution {
public:
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        if (find(wordList.begin(), wordList.end(), endWord) == wordList.end()) return 0;
        int L = beginWord.size();
        unordered_map<string, vector<int>> buckets;
        for (int idx = 0; idx < (int)wordList.size(); idx++) {
            string key = wordList[idx];
            for (int i = 0; i < L; i++) {
                char ch = key[i];
                key[i] = '*';
                buckets[key].push_back(idx);
                key[i] = ch;
            }
        }
        vector<bool> visited(wordList.size(), false);
        queue<string> q;
        q.push(beginWord);
        int steps = 1;
        while (!q.empty()) {
            for (int k = q.size(); k > 0; k--) {
                string word = move(q.front());
                q.pop();
                if (word == endWord) return steps;
                for (int i = 0; i < L; i++) {
                    char ch = word[i];
                    word[i] = '*';
                    auto it = buckets.find(word);
                    word[i] = ch;
                    if (it == buckets.end()) continue;
                    for (int nxt : it->second) {
                        if (!visited[nxt]) {
                            visited[nxt] = true;
                            q.push(wordList[nxt]);
                        }
                    }
                    buckets.erase(it);
                }
            }
            steps++;
        }
        return 0;
    }
};
