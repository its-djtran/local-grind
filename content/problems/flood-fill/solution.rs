impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let (sr, sc) = (sr as usize, sc as usize);
        let original = image[sr][sc];
        if original == color {
            return image;
        }
        let (m, n) = (image.len(), image[0].len());
        image[sr][sc] = color;
        let mut queue = VecDeque::from([(sr, sc)]);
        while let Some((r, c)) = queue.pop_front() {
            let neighbors = [
                (r + 1, c),
                (r.wrapping_sub(1), c),
                (r, c + 1),
                (r, c.wrapping_sub(1)),
            ];
            for (nr, nc) in neighbors {
                if nr < m && nc < n && image[nr][nc] == original {
                    image[nr][nc] = color;
                    queue.push_back((nr, nc));
                }
            }
        }
        image
    }
}
