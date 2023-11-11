class UnionFindWithPotential
  def initialize(size)
    @rank = Array.new(size, 0)
    @root = Array.new(size, &:itself)
    @diff = Array.new(size) { 0 }
  end

  def unite(x, y, w)
    # weight calculation for the original x and y
    w += weight(x)
    w -= weight(y)

    x = root(x)
    y = root(y)
    return false if x == y

    if @rank[x] < @rank[y]
      x, y = y, x
      w = -w
    end
    @rank[x] += 1 if @rank[x] == @rank[y]
    @root[y] = x
    @diff[y] = w

    true
  end

  def weight(x)
    root(x)
    @diff[x]
  end

  def diff(x, y)
    weight(y) - weight(x)
  end

  def root(x)
    if @root[x] == x
      x
    else
      r = root(@root[x])
      @diff[x] += @diff[@root[x]]
      @root[x] = r
    end
  end

  def same?(x, y)
    root(x) == root(y)
  end

  def debug
    p ["rank", @rank]
    p ["root", @root]
    p ["diff", @diff]
    puts "-" * 20
  end
end
