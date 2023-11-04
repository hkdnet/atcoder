def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis

dp = Array.new(M+1) { Float::MAX }
mins = {}
rs = N.times.map do |rid|
  c, p, *s = getis
  min = s.min
  sum = s.sum
  e = sum.to_f/s.size
  r = { cost: c, min:, sum:, e:, s:, cp: e/c}
  (1..min).each do |d|
    dp[M-d] = c if dp[M-d] > c
  end
  r
end

best = rs.max_by {|e| e[:cp] }

(M-1).downto(1) do |i|
  next if dp[i] == Float::MAX
  cnt = ((i * 1.0 * best[:s].size)/best[:sum]).ceil
  tmp = dp[i] + best[:cost] * cnt
  dp[0] = tmp if dp[0] > tmp
end

puts dp[0]

