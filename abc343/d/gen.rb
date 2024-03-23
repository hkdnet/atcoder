# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 2 * 10**5
T = 2 * 10**5

def rand_a
  randi(1, N+1)
end

def rand_b
  randi(1, 10)
end

puts "#{N} #{T}"
T.times do
  a =rand_a
  b = rand_b
  puts "#{a} #{b}"
end

