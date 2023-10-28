def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
tmp = N
loop do
  a, b, c = tmp.to_s.chars.map(&:to_i)
  if a * b == c
    break
  end
  tmp = tmp + 1
end

puts tmp
