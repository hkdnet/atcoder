# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end
a = randi(1, 10)
b = randi(1, 10)
k = randi(1, 10)

puts [a, b, k].join(" ")
