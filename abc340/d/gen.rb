# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 5
puts N
(N-1).times do
  a = randi(1, 100)
  b = randi(1, 100)
  x = randi(1, N)
  puts [a, b, x].join(" ")
end
