class UnionFind
  def initialize(size)
    @rank = Array.new(size, 0)
    @parent = Array.new(size, &:itself)
  end

  def unite(id_x, id_y)
    x_parent = get_parent(id_x)
    y_parent = get_parent(id_y)
    return if x_parent == y_parent

    if @rank[x_parent] > @rank[y_parent]
      @parent[y_parent] = x_parent
    else
      @parent[x_parent] = y_parent
      @rank[y_parent] += 1 if @rank[x_parent] == @rank[y_parent]
    end
  end

  def get_parent(id_x)
    @parent[id_x] == id_x ? id_x : (@parent[id_x] = get_parent(@parent[id_x]))
  end

  def same_parent?(id_x, id_y)
    get_parent(id_x) == get_parent(id_y)
  end
end

uf = UnionFind.new(N)

# 最小全域木を求めるアルゴリズム

# コストでソート
edges = M.times.map do
  u, v, w = getis
  [u - 1, v - 1, w]
end.sort_by {|e| e[2]}

ans = 0

edges.each do |u, v, w|
  # 閉路ができないようにしつつ
  if uf.same_parent?(u, v)
    next
  end

  # ひたすら足していく
  uf.unite(u, v)
  ans += w
  ans %= K
end

puts ans

