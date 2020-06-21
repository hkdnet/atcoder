n = 100000
puts n
puts n.times.map { rand(10000).to_s }.join(' ')
q = 100000
puts q
q.times do
  b = rand(10000)
  c = rand(10000)
  puts "#{b} #{c}"
end
