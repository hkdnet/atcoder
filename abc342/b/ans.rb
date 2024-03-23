def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
P = getis
h = P.each.with_index.to_h
Q = geti
Q.times do
  a, b = getis
  if h[a] > h[b]
    puts b
  else
    puts a
  end
end
