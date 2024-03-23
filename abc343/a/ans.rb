def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

A, B = getis

c = A + B
if c == 9
  puts 1
else
  puts(c + 1)
end
