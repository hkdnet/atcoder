def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
pos = N.times.to_h do |i|
  [1 << i, getis]
end
memo = Hash.new do |h, k|
  h[k] = {}
end
dist = ->(a, b) {
  if a > b
    dist[b, a]
  else
    memo[a][b] ||=
      begin
        ax, ay = pos[a]
        bx, by = pos[b]
        d = (bx-ax)**2
        d += (by-ay)**2
        Math.sqrt(d)
      end
  end
}
ans = Float::INFINITY

all_state = (1<<N)-1
N.times do |i|
  start = 1 << i
  dp = Hash.new do |h, k|
    h[k] = Hash.new(Float::INFINITY)
  end
  dp[start][start] = 0

  (1<<N).times do |s|
    dp[s].each do |pos, cost|
      N.times do |i|
        dest = 1 << i
        next if pos == dest
        next if s & dest != 0

        nx = s | dest
        nv = cost + dist[pos, dest]
        dp[nx][dest] = nv if dp[nx][dest] > nv
      end
    end
  end

  tmp = dp[all_state].map do |pos, cost|
    next Float::INFINITY if pos == start

    cost + dist[pos, start]
  end.min

  ans = tmp if tmp < ans
end

puts ans
