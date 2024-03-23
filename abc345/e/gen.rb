# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 10
K = 5
puts "#{N} #{K}"
N.times do
  c = randi(1, 3)
  v = randi(1, 10)
  puts "#{c} #{v}"
end
