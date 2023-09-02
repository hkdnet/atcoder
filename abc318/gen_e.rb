N = 3 * 10**5
puts N
arr = N.times.map do |i|
  i % 2 == 0 ? 1 : 2
end

puts arr.join(" ")
