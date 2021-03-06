use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        pab_list: [(usize,usize,usize); q]
    }
    let mut uf = UnionFind::new(n);
    for pab in pab_list {
        if pab.0 == 0 {
            uf.union(pab.1, pab.2);
        } else {
            let ans = if uf.same(pab.1, pab.2) { "Yes" } else { "No" };
            println!("{}", ans);
        }
    }
}

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}
impl UnionFind {
    fn new(len: usize) -> UnionFind {
        return UnionFind {
            parents: (0..len).map(|i| i).collect(),
            ranks: vec![0; len],
        };
    }
    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            let root_of_parent = self.root(self.parents[x]);
            self.parents[x] = root_of_parent;
            root_of_parent
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y {
            return;
        }
        if self.ranks[root_x] < self.ranks[root_y] {
            self.parents[root_x] = root_y;
        } else {
            self.parents[root_y] = root_x;
            if self.ranks[root_x] == self.ranks[root_y] {
                self.ranks[root_x] += 1;
            }
        }
    }
}
