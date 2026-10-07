class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        int counts[26] = {};
        for (char t : tasks) counts[t - 'A']++;
        int f = *max_element(counts, counts + 26);
        int k = count(counts, counts + 26, f);
        return max((int)tasks.size(), (f - 1) * (n + 1) + k);
    }
};
