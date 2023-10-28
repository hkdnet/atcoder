def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N, M = getis

ps = getis
h = {}
ps.each do |p|
  h[p] ||= 0
  h[p] += 1
end

keys = h.keys.sort

l = 0
r = 1

ans = 0
while l < keys.size
  while keys[r] && keys[r] < keys[l] + M
    r += 1
  end

  tmp = (l...r).map { |e| h[keys[e]] }.sum
  ans = tmp if ans < tmp

  break if r >= keys.size

  l += 1
end

puts(ans)
