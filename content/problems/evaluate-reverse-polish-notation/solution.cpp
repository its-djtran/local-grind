class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        vector<long long> st;
        for (const string& token : tokens) {
            if (token == "+" || token == "-" || token == "*" || token == "/") {
                long long b = st.back(); st.pop_back();
                long long a = st.back(); st.pop_back();
                switch (token[0]) {
                    case '+': st.push_back(a + b); break;
                    case '-': st.push_back(a - b); break;
                    case '*': st.push_back(a * b); break;
                    default:  st.push_back(a / b); break;
                }
            } else {
                st.push_back(stoll(token));
            }
        }
        return (int)st[0];
    }
};
