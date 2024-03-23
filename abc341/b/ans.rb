def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis
t = (N-1).times.map do
  getis
end

t.each.with_index do |(s, t), i|
  m = A[i] / s
  A[i+1] += t * m
end

puts A[N-1]
