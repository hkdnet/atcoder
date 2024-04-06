# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 100
puts N
N.times.map do
  x = randi(1, 100)
  y = randi(1, 100)
  puts "#{x} #{y}"
end
