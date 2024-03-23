def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, K = getis
A = getis
h = {}
A.each do |a|
  h[a] = true if a <= K
end
ans = K * (K+1) / 2
ans -= h.keys.sum
puts ans
