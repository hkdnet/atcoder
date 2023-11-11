# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 20
s = N.times.map { %w[AB BC CA A B C ABC].sample }.join("")
puts s
