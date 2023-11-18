# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

N = 10
M = randi(1, 6)
S = N.times.map { %W[A B].sample }.join("")
l = randi(0, N - M)
T = S[l, M]
# arr.shuffle
# arr.sample
# arr.sample(n)

puts "#{N} #{M}"
puts S
puts T
