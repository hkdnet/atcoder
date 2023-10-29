N = 100
M = (1..10).to_a.sample
a = []
N.times do
  a << rand(20) + 1
end

puts "#{N} #{M}"
puts a.join(" ")

