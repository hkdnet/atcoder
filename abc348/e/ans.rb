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

# 全方位木DP, rerooting

# 配列 sum_c[i]: 頂点 i の部分木の頂点 x について C[x] の総和
sum_c = []
# 配列 sum_d[i]: 頂点 i の部分木の頂点 x について C[x] x d(i, x) の総和
sum_d = []
dfs = ->(v, parent) {
  sum_c[v] = C[v]
  sum_d[v] = 0
  E[v].each do |t, _|
    next if t == parent
    dfs[t, v]
    sum_c[v] += sum_c[t]
    sum_d[v] += sum_c[t] + sum_d[t]
  end
}

dfs[0, -1]

# 配列 f[i]: f(i)
f = []

# v: 今の頂点
# p_sum_c: v の部分木以外の頂点 x について C[x] の総和
# p_sum_d: v の部分木以外の頂点 x について C[x] x d(v, x) の総和
rerooting = ->(v, parent, p_sum_c, p_sum_d) {
  f[v] = sum_d[v] + p_sum_d
  E[v].each do |t, _|
    next if t == parent

    nx_sum_c = p_sum_c + sum_c[v] - sum_c[t]
    nx_sum_d = p_sum_d + sum_d[v] - (sum_d[t] + sum_c[t]) + nx_sum_c
    rerooting[t, v, nx_sum_c, nx_sum_d]
  end
}

rerooting[0, -1, 0, 0]

ans = f.min
puts ans
