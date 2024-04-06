# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
N = 10**5
puts N
(N-1).times do |a|
  puts "#{a+1} #{a+2}"
end
C = N.times.map do
  randi(1, 10**9)
end
puts C.join(" ")
