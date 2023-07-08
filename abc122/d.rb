N = gets.chomp.to_i
MOD = 10**9 + 7

A = 0
C = 1
G = 2
T = 3
dp = [
  Array.new(N+1) { 0 },
  Array.new(N+1) { 0 },
  Array.new(N+1) { 0 },
  Array.new(N+1) { 0 },
]

[A, C, G, T].each do |c|
  dp[c][1] = 1
end

# bad patterns:
#  * AGC
#  * GAC
#  * ACG
#  * ATGC
#  * AGGC
#  * AGTC
2.upto(N) do |idx|
  # A
  dp[A][idx] = dp[A][idx - 1] + dp[C][idx - 1] + dp[G][idx - 1] + dp[T][idx - 1]
  dp[A][idx] %= MOD
  # C
  dp[C][idx] = dp[A][idx - 1] + dp[C][idx - 1] + dp[G][idx - 1] + dp[T][idx - 1]
  dp[C][idx] -= dp[A][idx - 2] if idx - 2 >= 0 # AGC
  dp[C][idx] -= dp[G][idx - 2] if idx - 2 >= 0 # GAC
  # ATGC AGGC AGTC
  if idx >= 3
    dp[G][idx] -= dp[A][idx - 3] * 3 # ATGC
  end
  dp[C][idx] %= MOD
  # G
  dp[G][idx] = dp[A][idx - 1] + dp[C][idx - 1] + dp[G][idx - 1] + dp[T][idx - 1]
  dp[G][idx] -= dp[A][idx - 2] if idx - 2 >= 0 # ACG
  dp[G][idx] %= MOD
  # T
  dp[T][idx] = dp[A][idx - 1] + dp[C][idx - 1] + dp[G][idx - 1] + dp[T][idx - 1]
  dp[T][idx] %= MOD
end

ans = 0
[A, C, G, T].each do |c|
  ans += dp[c][N]
  ans %= MOD
end

puts ans
