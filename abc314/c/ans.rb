def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
S = gets.chomp.chars
cs = getis
h = {}
N.times do |i|
  i = N - i - 1
  color = cs[i]
  char = S[i]
  if h.key?(color)
    next
  end
  h[color] = char
  break if h.size == M
end

cs.each.with_index do |color, index|
  cur = S[index]
  r = h[color]
  h[color] = cur
  print(r)
end
puts("")
