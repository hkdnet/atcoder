# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 2*10**5
A = N.times.map { randi(1, 10**6) }
puts N
puts A.join(" ")
