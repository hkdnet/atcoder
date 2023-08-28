x1, y1, x2, y2 = gets.chomp.split(" ").map(&:to_i)

v = [x2 - x1, y2 - y1]
x3 = x2 - v[1]
y3 = y2 + v[0]
x4 = x3 - v[0]
y4 = y3 - v[1]

puts "#{x3} #{y3} #{x4} #{y4}"
