def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
a, _, c = S.split("|", 3)
puts "#{a}#{c}"
