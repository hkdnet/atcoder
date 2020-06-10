n = 2 * 10**5
puts n
puts n.times.map { rand(10**6) }.to_a.shuffle.join(' ')

