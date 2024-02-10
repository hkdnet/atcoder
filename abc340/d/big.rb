# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 2 * 10**5
puts N
(N-1).times do
  a = randi(100, 1000)
  b = randi(1, 10)
  x = randi(1, N)
  puts [a, b, x].join(" ")
end
