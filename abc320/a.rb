def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

A, B = getis

puts(A**B + B**A)
