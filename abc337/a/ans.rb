def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
a = []
b = []
N.times do
  aa, bb = getis
  a << aa
  b << bb
end

as = a.sum
bs = b.sum
if as == bs
  puts "Draw"
else
  if as > bs
    puts "Takahashi"
  else
    puts "Aoki"
  end
end
