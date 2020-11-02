struct UnionFind {
  par: Vec<usize>
}

impl UnionFind {
  fn new(n: usize) -> UnionFind {
    let mut par: Vec<usize> = vec![0;n];
    for i in 0..n { par[i] = i; }
    UnionFind { par }
  }

  fn root(&mut self, x: usize) -> usize {
    if self.par[x] == x { x }
    else {
      self.par[x] = self.root(self.par[x]);
      self.par[x]
    }
  }

  fn unite(&mut self, x: usize, y: usize) {
    let rx = self.root(x);
    let ry = self.root(y);
    if rx != ry {
      self.par[rx] = ry;
    }
  }

  fn same (&mut self, x: usize, y: usize) -> bool {
    self.root(x) == self.root(y)
  }
}
