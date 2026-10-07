class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        first_row = any(v == 0 for v in matrix[0])
        first_col = any(matrix[i][0] == 0 for i in range(m))
        for i in range(1, m):
            row = matrix[i]
            for j in range(1, n):
                if row[j] == 0:
                    row[0] = 0
                    matrix[0][j] = 0
        top = matrix[0]
        for i in range(1, m):
            row = matrix[i]
            if row[0] == 0:
                for j in range(1, n):
                    row[j] = 0
            else:
                for j in range(1, n):
                    if top[j] == 0:
                        row[j] = 0
        if first_row:
            for j in range(n):
                top[j] = 0
        if first_col:
            for i in range(m):
                matrix[i][0] = 0
