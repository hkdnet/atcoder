def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis

arr = Array.new(N+1) { 0 }
A.each.with_index do |a, i|
  arr[i+1] = a
end
N.times do |i|
  arr[i+1] += arr[i]
end
base = 0
arr.each do |a|
  base = a if a < base
end

puts (arr.last - base)
