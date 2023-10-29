# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 5
puts N
N.times do
  a = rand(5) + 1
  b = rand(5) + 1
  puts "#{a} #{b}"
end
