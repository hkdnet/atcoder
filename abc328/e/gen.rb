# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 5
arr = 5.times.map {|e| e + 1}
puts "#{N} #{N*(N-1)/2} 100000000"
arr.combination(2).each do |a, b|
  puts "#{a} #{b} #{randi(0, 30)}"
end
