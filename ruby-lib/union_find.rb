class UnionFind
  def initialize(size)
    @rank = Array.new(size, 0)
    @root = Array.new(size, &:itself)
  end

  def unite(x, y)
    x_root = root(x)
    y_root = root(y)
    return if x_root == y_root

    if @rank[x_root] > @rank[y_root]
      @root[y_root] = x_root
    else
      @root[x_root] = y_root
      @rank[y_root] += 1 if @rank[x_root] == @rank[y_root]
    end
  end

  def root(x)
    @root[x] == x ? x : (@root[x] = root(@root[x]))
  end

  def same?(x, y)
    root(x) == root(y)
  end
end
