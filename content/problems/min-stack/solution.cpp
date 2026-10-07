class MinStack {
    vector<pair<int, int>> st; // {value, minimum of the stack up to and including this value}

public:
    MinStack() {}

    void push(int val) {
        st.emplace_back(val, st.empty() ? val : min(val, st.back().second));
    }

    void pop() {
        st.pop_back();
    }

    int top() {
        return st.back().first;
    }

    int getMin() {
        return st.back().second;
    }
};
