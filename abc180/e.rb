# bit DP

# highly inspired by https://atcoder.jp/contests/abc180/submissions/43460522

N = gets.chomp.to_i
points = N.times.map do |i|
  x, y, z = gets.chomp.split(" ").map(&:to_i)
  { id: i, x: x, y: y, z: z }
end

def distance(a, b)
  (a[:x] - b[:x]).abs + (a[:y] - b[:y]).abs + [0, b[:z] - a[:z]].max
end

t = Hash.new { |h, k| h[k] = {} }

N.times do |i|
  N.times do |j|
    t[i][j] = distance(points[i], points[j])
  end
end

# dp[i][S]: cur = i, visited = S
dp = Array.new(N) { Array.new(1 << N, Float::INFINITY) }
dp[0][0] = 0

(1 << N).times do |i|
  next if i & 1 == 1 # The journey must end at the 1st city. So we can stop this search since the 1st city is visited

  N.times do |j|
    next if dp[j][i] == Float::INFINITY

    N.times do |k|
      nx = dp[j][i] + t[j][k]
      nx_state = i | (1 << k)
      dp[k][nx_state] = [dp[k][nx_state], nx].min
    end
  end
end

puts dp[0][(1 << N) - 1]
