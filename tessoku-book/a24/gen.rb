# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 5
A = N.times.map do
  randi(1, 5)
end

puts N
puts A.join(" ")
