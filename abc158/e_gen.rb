v = rand(10000)
n = v.to_s.size
prime = [2, 3, 5, 11, 17].shuffle.first

puts "#{n} #{prime}"
puts v
