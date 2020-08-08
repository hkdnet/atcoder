n = 5000
k = 10**9
ps = (1..5000).to_a.shuffle
cs = n.times.map { rand(100) }

puts "#{n} #{k}"
puts ps.join(' ')
puts cs.join(' ')
