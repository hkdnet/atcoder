N, D, P = gets.chomp.split(" ").map(&:to_i)
fs = gets.chomp.split(" ").map(&:to_i)

ans = 0
fs.sort_by{|e| -e}.each_slice(D) do |chunk|
  s = chunk.sum
  ans += [s, P].min
end

puts ans
