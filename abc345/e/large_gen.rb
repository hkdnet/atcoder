# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 2*10**5
K = 500
puts "#{N} #{K}"
N.times do
  c = randi(1, 1000)
  v = randi(1, 10)
  puts "#{c} #{v}"
end
