# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

max = 2 * 10**5
H = max
W = (max / H).to_i
K = [H, W].max
puts "#{H} #{W} #{K}"
def rand_c
  %w[o x .].sample
end
H.times do
  line = W.times.map { rand_c }
  puts line.join("")
end


# arr.shuffle
# arr.sample
# arr.sample(n)
