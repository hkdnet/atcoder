# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)
cs = ("a".."e").to_a
T = 10.times.map { cs.sample }.join("")
N = 10
puts T
puts N

N.times do
  a = 5
  arr = a.times.map do
    (cs + ["f"]).sample(randi(1, 4)).join("")
  end
  puts "#{a} #{arr.join(" ")}"
end
