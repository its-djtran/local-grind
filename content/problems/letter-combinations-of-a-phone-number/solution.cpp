class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};
        static const string keypad[10] = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};
        vector<string> combos = {""};
        for (char d : digits) {
            vector<string> next;
            for (const string& c : combos)
                for (char letter : keypad[d - '0'])
                    next.push_back(c + letter);
            combos = move(next);
        }
        return combos;
    }
};
