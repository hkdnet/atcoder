def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }


N, L, R = getis

A = (1..N).to_a
A[L-1..R-1] = A[L-1..R-1].reverse
puts A.join(" ")
