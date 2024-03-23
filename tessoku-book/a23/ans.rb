def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
cs = M.times.map do
  getis.join("").to_i(2)
end
INF = M+1
dp = Hash.new(INF)
dp[0] = 0
(1 << N).times do |s|
  cur = dp[s]
  next if cur == INF # unreachable
  cs.each do |c|
    nx = s | c
    nv = cur + 1
    dp[nx] = nv if dp[nx] > nv
  end
end

ans = dp[(1<<N)-1]
ans = -1 if ans == INF
puts ans
