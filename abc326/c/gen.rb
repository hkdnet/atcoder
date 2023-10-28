N = 5
M = [1, 2, 3, 4, 5].shuffle.first
a = []
N.times do
  a << rand(20)
end

puts "#{N} #{M}"
puts a.join(" ")

