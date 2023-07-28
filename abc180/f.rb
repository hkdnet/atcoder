n, m, l = gets.chomp.split(" ").map(&:to_i)

MOD = 10**9 + 7

fact = [1, 1]
inv = [0, 1]
fact_inv = [1, 1]

2.upto(300) do |i|
  fact[i] = fact[i - 1] * i % MOD
  inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD
  fact_inv[i] = fact_inv[i-1] * inv[i] % MOD
end

nck = ->(n, k) {
  fact[n] * (fact_inv[k] * fact_inv[n-k] % MOD) % MOD
}


# サイズLのグラフを作るためには
#   次数 1 の頂点 x 2
#   次数 2 の頂点 x (L-2)
# なので頂点L、辺 L-1 使う。
# このグラフの構築は頂点の選び方が C(n, l) 通り。並べ方で l! / 2 通り
ans = nck[n, l] * (fact[l] / 2)
ans %= MOD

# 残りをどうするかを考える。
n = n - l
m = l - 1

if m < n - 1
  rest = n - 1 - m
  # 頂点が余るので頂点1のグラフができる。
  ans += nck[n, rest]
  ans %= MOD
  n = m + 1
end

# 最大連結数Lを超えない程度に N 個の頂点と N-1 個の辺を消化すればよいがここどうすんの～～～

puts ans
