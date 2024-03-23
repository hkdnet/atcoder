# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

N = randi(3, 6)
S = N.times.map do
  %w[a b c d e f].sample
end.join("")
puts N
puts S
