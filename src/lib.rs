pub struct Dsu(Vec<isize>, usize);
impl Dsu {
  /// 頂点数 `n` の辺の無い無向グラフを初期化する
  pub fn new(n: usize) -> Self { Self(vec![-1; n], n) }

  /// 頂点 `v` が属する連結成分の代表となる頂点を求める
  /// 計算量: `O(α(n))` amortized
  pub fn leader(&mut self, mut v: usize) -> usize { let k = self.0[v]; if k >= 0 { let j = self.leader(k as usize); self.0[v] = j as isize; v = j; }; v }

  /// 頂点 `u, v` 間に辺を貼る
  /// この操作をした時点で、 `u, v` が初めて連結になる場合は `true` 、そうでない（既に連結だった）場合は `false` を返す
  /// 計算量: `O(α(n))` amortized
  pub fn merge(&mut self, mut u: usize, mut v: usize) -> bool { u = self.leader(u); v = self.leader(v); u != v && { if self.0[u] > self.0[v] { let k = u; u = v; v = k; }; self.0[u] += self.0[v]; self.0[v] = u as isize; true } }

  /// 頂点 `u, v` が連結かどうかを返す
  /// 計算量: `O(α(n))` amortized
  pub fn same(&mut self, u: usize, v: usize) -> bool { self.leader(u) == self.leader(v) }

  /// 頂点 `v` が属する連結成分の頂点数を返す
  /// 計算量: `O(α(n))` amortized
  pub fn size(&mut self, mut v: usize) -> usize { v = self.leader(v); -self.0[v] as usize }

  /// すべての連結成分を代表となる頂点の昇順で返す
  /// 計算量: `O(n α(n))`
  pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut s = vec![vec![]; self.1]; for i in 0 .. self.1 { s[self.leader(i)].push(i) }; s.into_iter().filter(|g| g.len() > 0 ).collect() }
}