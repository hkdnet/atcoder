def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


A, B, D = getis

arr = []
tmp = A
loop do
  arr << tmp
  break if tmp == B
  tmp += D
end

puts arr.join(" ")
