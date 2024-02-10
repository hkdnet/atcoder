# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 60
x1 = randi(0, N)
x2 = randi(0, N)
y1 = randi(0, N)
y2 = randi(0, N)
if x1 == x2 && y1 == y2
  exit 1
end

puts N

N.times do |x|
  N.times do |y|
    if (x == x1 && y == y1) || (x == x2 && y == y2)
      print "P"
    else
      print "."
    end
  end
  puts ""
end
