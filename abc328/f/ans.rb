def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, Q = getis
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

uf = UnionFindWithPotential.new(N)
ans = []
Q.times do |q|
  q = q + 1
  a, b, d = getis
  a = a - 1
  b = b - 1

  if uf.same?(a, b)
    if uf.diff(a, b) == d
      ans << q
    end
  else
    uf.unite(a, b, d)
    ans << q
  end
end

puts ans.join(" ")
