def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
ss = Set.new
slimes = Hash.new {|h, k| h[k] = 0 }
N.times do
  s, c = getis
  slimes[s] = c
  ss << s if c > 1
end

until ss.empty?
  s = ss.first
  ss.delete(s)
  h = slimes[s] / 2
  slimes[s] -= h * 2
  nx = s * 2
  slimes[nx] += h
  ss << nx if slimes[nx] > 1
end

puts(slimes.values.sum)
