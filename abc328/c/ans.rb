def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, Q = getis
S = gets.chomp.chars
acc = Array.new(N) { 0 }
(N-1).times do |i|
  d = S[i] == S[i+1] ? 1 : 0
  acc[i+1] = acc[i] + d
end

Q.times do
  l, r = getis
  puts acc[r-1] - acc[l-1]
end
