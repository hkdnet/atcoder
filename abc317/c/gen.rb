N = 10
pairs = 1.upto(N).to_a.combination(2)
puts "#{N} #{pairs.size}"
pairs.each do |a, b|
  puts "#{a} #{b} 1"
end
