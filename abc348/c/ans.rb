def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
h = Hash.new do |h, k|
  h[k] = Float::INFINITY
end
N.times do
  a, c = getis
  h[c] = a if a < h[c]
end

# p h

ans = 0
h.each do |c, a|
  if ans < a
    ans = a
  end
end

puts ans
