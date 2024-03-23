def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
N = S.size
ans = N * (N-1) / 2
cnt = S.chars.tally
delta = 0
cs = 0
cnt.each do |c, v|
  d = v * (v-1) / 2
  if d > 0
    delta += d
    cs += 1
  end
end

ans -= delta
if delta > 0
  ans += 1
end
puts ans
