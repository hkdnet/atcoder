# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

H = 500
W = 500
N = 500
T = (%w[L R D U] * (N / 4)).join("")

puts "#{H} #{W} #{N}"
puts T
H.times do |i|
  if i == 0 || i == H - 1
    puts "#" * W
  else
    puts "#" + ("." * (W-2)) + "#"
  end
end
