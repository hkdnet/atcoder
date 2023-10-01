def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, X = getis
A = getis
as = A.sum

f = false
0.upto(100) do |e|
  arr = A + [e]
  s = as + e
  mi, ma = arr.minmax
  score = s - mi - ma
  if score >= X
    puts e
    f = true
    break
  end
end
if !f
  puts -1
end
