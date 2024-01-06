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

def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
A = getis
A.unshift(nil)
edges = Hash.new { |h, k| h[k] = {} } # from -> to
uf = UnionFind.new(N + 1)
M.times do
  u, v = getis
  if A[v] < A[u]
    u, v = v, u
  end
  # A[u] <= A[v]
  edges[u][v] = true
  if A[u] == A[v]
    edges[v][u] = true
    uf.unite(u, v)
  end
end

merger = {}
(0..N).map do |i|
  r = uf.root(i)
  merger[r] ||= []
  merger[r] << i
end
merger.each do |_, v|
  next if v.size <= 1

  all = {}
  v.each do |e|
    edges[e].keys.each do |k|
      all[k] = true
    end
  end
  v.each do |e|
    edges[e] = all
  end
end

q = []
q << 1
cnts = Array.new(N+1) { 0 }
cnts[1] = 1

until q.empty?
  cur = q.pop
  if cur == N
    next
  end

  cur_v = A[cur]
  edges[cur].keys.each do |nx|
    nx_cnt = cnts[cur]
    nx_cnt += 1 unless cur_v == A[nx]

    if cnts[nx] < nx_cnt
      cnts[nx] = nx_cnt
      q << nx
    end
  end
end

puts cnts[N]
