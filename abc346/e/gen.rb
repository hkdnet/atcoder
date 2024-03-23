# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
H = 5
W = 4
M = 10
puts "#{H} #{W} #{M}"

M.times do
  ty = randi(1, 3)
  if ty == 1
    a = randi(1, H+1)
  else
    a = randi(1, W+1)
  end
  x = randi(0, 4)
  puts "#{ty} #{a} #{x}"
end
