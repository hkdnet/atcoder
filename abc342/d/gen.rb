# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 5
a = N.times.map do
  randi(0, 10)
end

puts N
puts a.join(" ")
