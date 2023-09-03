pub struct Dsu {
  // 根（値が負）の場合は連結成分の要素数、子孫の場合は親の番号を表す
  p: Vec<isize>,
  // 頂点数
  n: usize,
}

impl Dsu {
  /// 頂点数 `n` の辺の無い無向グラフを初期化する
  /// 
  /// 計算量: `O(n)`
  pub fn new(n: usize) -> Self {
    Self {
      p: vec![-1; n],
      n,
    }
  }

  /// 頂点 `v` が属する連結成分の代表となる頂点を求める
  /// 
  /// 計算量: `O(α(n))` amortized
  pub fn leader(&mut self, mut v: usize) -> usize {
    assert!(v < self.n);
    let mut decendants = vec![];
    // v が根になるまで遡る
    while self.p[v] >= 0 {
      decendants.push(v);
      v = self.p[v] as usize;
    }
    // 子孫の親を根に付け替える (path compression)
    for u in decendants {
      self.p[u] = v as isize;
    }
    v
  }

  /// 頂点 `u, v` 間に辺を貼る
  /// この操作をした時点で、 `u, v` が初めて連結になる場合は `true` 、そうでない（既に連結だった）場合は `false` を返す
  /// 
  /// 計算量: `O(α(n))` amortized
  pub fn merge(&mut self, mut u: usize, mut v: usize) -> bool {
    u = self.leader(u);
    v = self.leader(v);
    if u == v {
      return false;
    }
    // 要素数が大きい方に小さい方を付ける (union by size)
    if self.p[u] > self.p[v] {
      std::mem::swap(&mut u, &mut v);
    };
    self.p[u] += self.p[v];
    self.p[v] = u as isize;
    true
  }

  /// 頂点 `u, v` が連結かどうかを返す
  /// 
  /// 計算量: `O(α(n))` amortized
  pub fn same(&mut self, u: usize, v: usize) -> bool {
    self.leader(u) == self.leader(v)
  }

  /// 頂点 `v` が属する連結成分の頂点数を返す
  /// 
  /// 計算量: `O(α(n))` amortized
  pub fn size(&mut self, v: usize) -> usize {
    let r = self.leader(v);
    -self.p[r] as usize
  }

  /// すべての連結成分を代表となる頂点の昇順で返す
  /// 
  /// 計算量: `O(n α(n))`
  pub fn groups(&mut self) -> Vec<Vec<usize>> {
    let mut s = vec![vec![]; self.n];
    for v in 0 .. self.n {
      s[self.leader(v)].push(v);
    }
    s.into_iter().filter(|g| !g.is_empty() ).collect()
  }
}