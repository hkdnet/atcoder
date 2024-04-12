def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N = geti
E = Hash.new do |h, k|
  h[k] = {}
end
(N-1).times do
  a, b = getis1
  E[a][b] = true
  E[b][a] = true
end
C = getis
SUM = C.sum

# 木の重心を考える
# f(x) = sum of Ci*d(x, i) なので頂点 i が C[i] だけ重いと考えて良い

# subtree_sum[i]: 頂点 i の部分木を x として C[x] の合計値
subtree_sum = []
dfs = ->(cur, parent) {
  subtree_sum[cur] = C[cur] #
  E[cur].each do |to, _|
    next if parent == to
    subtree_sum[cur] += dfs[to, cur]
  end
  subtree_sum[cur]
}
dfs[0, -1]

# すべての部分木の重さが SUM / 2 より低ければ重心として良い
find_centroid = ->(cur, parent) {
  centroid_p = true
  E[cur].each do |to, _|
    next if to == parent

    tmp = find_centroid[to, cur]
    return tmp if tmp != -1

    if subtree_sum[to] > SUM / 2
      centroid_p = false
    end
  end
  # 親側の部分木を考える
  parent_sum = SUM - subtree_sum[cur]
  if parent_sum > SUM / 2
    centroid_p = false
  end

  centroid_p ? cur : -1
}

centroid = find_centroid[0, -1]

# シンプルな深さ計算
dist = []
calc_dist = ->(cur, parent, d) {
  dist[cur] = d
  E[cur].each do |to, _|
    next if to == parent

    calc_dist[to, cur, d + 1]
  end
}
calc_dist[centroid, -1, 0]

ans = 0
dist.each.with_index do |d, i|
  ans += d * C[i]
end
puts ans

