# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 5000
arr = N.times.map {|e| randi(0, 1200)}

puts N
puts arr.join(" ")
