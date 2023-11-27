# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

puts randi(1, 1000)
# arr.shuffle
# arr.sample
# arr.sample(n)
