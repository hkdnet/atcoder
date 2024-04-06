# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

N = 7
H = 10
W = 10
puts "#{N} #{H} #{W}"

N.times do
  a = randi(1, 10)
  b = randi(1, 10)
  puts "#{a} #{b}"
end
