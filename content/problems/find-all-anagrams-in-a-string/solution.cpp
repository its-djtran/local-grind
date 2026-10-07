class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int m = p.size(), n = s.size();
        if (m > n) return {};
        array<int, 26> diff{}; // window count minus p count, per letter
        for (int i = 0; i < m; i++) {
            diff[p[i] - 'a']--;
            diff[s[i] - 'a']++;
        }
        int bad = ranges::count_if(diff, [](int d) { return d != 0; }); // letters whose counts disagree
        vector<int> out;
        if (bad == 0) out.push_back(0);
        for (int i = m; i < n; i++) {
            int add = s[i] - 'a', rem = s[i - m] - 'a';
            if (add != rem) {
                int d = diff[add]++;
                bad += (d == 0) - (d == -1);
                d = diff[rem]--;
                bad += (d == 0) - (d == 1);
            }
            if (bad == 0) out.push_back(i - m + 1);
        }
        return out;
    }
};
