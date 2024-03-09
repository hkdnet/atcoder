def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

arr = []
while (line = gets)
  a = line.chomp.to_i
  arr << a
end

arr.reverse_each do |a|
  puts a
end
