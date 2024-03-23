def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

_ = gets
A = getis
b = A.each_cons(2).map do |a1, a2|
  a1 * a2
end

puts b.join(" ")
