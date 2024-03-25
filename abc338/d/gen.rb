# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 5
M = rand(1..N)
X = (1..N).to_a.sample(M)

puts "#{N} #{M}"
puts X.join(" ")
