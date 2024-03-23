# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

az = ("a".."f").to_a
N = 6
cs = N.times.map do
  az.sample
end

S = cs.join("")
puts S
