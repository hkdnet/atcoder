# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 5
b = N.times.map do
  x = randi(1, 10)
  y = randi(1, 10)
  [x, y]
end
puts N
b.each do |x, y|
  puts "#{x} #{y}"
end
