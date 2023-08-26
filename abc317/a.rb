N, H, X = gets.chomp.split(' ').map(&:to_i)
ps = gets.chomp.split(' ').map(&:to_i)

i = ps.find_index do |p|
  H + p >= X
end

puts i + 1
