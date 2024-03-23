# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 5
S = N.times.map do
  randi(0, 2)
end.join("")
C = N.times.map do
  randi(1, 100)
end

puts N
puts S
puts C.join(" ")
