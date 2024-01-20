# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

max = 2 * (10**5)
H = 500
W = (max / H).to_i
K = H / 2
puts "#{H} #{W} #{K}"
def rand_c
  %w[.].sample
end
H.times do
  line = ""
  while line.size < W
    line += ("." * )
  end
  line = W.times.map { rand_c }
  puts line.join("")
end


# arr.shuffle
# arr.sample
# arr.sample(n)
