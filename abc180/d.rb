x, y, a, b = gets.chomp.split(" ").map(&:to_i)

ex = 0
t = x
loop do
  t1 = t * a
  t2 = t + b
  if t2 < t1
    ex += (y - 1 - t) / b
    break
  end

  break if t1 >= y

  ex += 1
  t = t1
end

puts ex

