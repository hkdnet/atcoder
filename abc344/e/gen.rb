# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 2 * 10**5
Q = 2 * 10**5
A = (1..10**6).to_a.sample(N)

puts N
puts A.join(" ")
puts Q
h = {}
A.each do |a|
  h[a] = true
end

Q.times do
  ty = randi(1, 3)
  if ty == 1
    x = h.keys.sample
    y = nil
    while y.nil?
      y = randi(10**6, 10**8)
      y = nil if h.key?(y)
    end
    puts "1 #{x} #{y}"
  else
    x = h.keys.sample
    puts "2 #{x}"
    h.delete(x)
  end
end
