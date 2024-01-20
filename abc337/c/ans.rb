def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis
h = A.each.with_index(1).to_h

cur = -1
ans = []
N.times do
  cur = h[cur]
  ans << cur
end

puts ans.join(" ")
