impl Solution {
    pub fn max_profit(p: Vec<i32>) -> i32 {
        let (mut p1, mut p2, mut max) = (0, 1, 0);

        while p1 < p.len() - 1 && p2 < p.len() {
            if p[p1] < p[p2] {
                let profit = p[p2] - p[p1];
                if  profit > max{
                    max = profit;
                }
                p2 += 1;
            } else {
                p1 = p2;
                p2 += 1;
            }
        }
        max
    }
}