# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 2*10**5
M = N
X = (1..N).to_a.shuffle

puts "#{N} #{M}"
puts X.join(" ")
