class Codec {
public:
    string encode(vector<string>& strs) {
        string out;
        for (const string& s : strs) {
            out += to_string(s.size());
            out += '#';
            out += s;
        }
        return out;
    }

    vector<string> decode(string s) {
        vector<string> out;
        size_t i = 0;
        while (i < s.size()) {
            size_t j = s.find('#', i);
            size_t n = stoul(s.substr(i, j - i));
            out.push_back(s.substr(j + 1, n));
            i = j + 1 + n;
        }
        return out;
    }
};

// Your Codec object will be instantiated and called as such:
// Codec ser, deser;
// vector<string> ans = deser.decode(ser.encode(strs));
