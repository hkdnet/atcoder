# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

s = "abcde"
q = 4
puts s.size
puts s
puts q
q.times do
  a, b = s.chars.sample(2)
  puts "#{a} #{b}"
end
