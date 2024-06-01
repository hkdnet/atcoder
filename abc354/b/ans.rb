def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N = geti
h = N.times.to_h do
  n, a = getl.split(" ")
  [n, a.to_i]
end
T = h.values.sum
idx = T % N
ans = h.keys.sort[idx]

puts ans
