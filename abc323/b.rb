def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N = geti

arr = N.times.map do |n|
  id = n + 1
  l = gets.chomp.split("").count {|e| e == 'x' }
  [l, id]
end

puts (
  arr.sort_by(&:first).map(&:last).join(" ")
)
