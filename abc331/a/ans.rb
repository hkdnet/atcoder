def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

M, D = getis
y, m, d = getis

d += 1
if d > D
  d = 1
  m += 1
  if m > M
    m = 1
    y += 1
  end
end

puts([y, m, d].join(" "))
