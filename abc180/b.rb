_ = gets
x = gets.chomp.split(" ").map(&:to_f)

def m(x)
  x.map(&:abs).sum
end

def e(x)
  Math.sqrt(x.map {|e| e **2 }.sum)
end

def c(x)
  x.map(&:abs).max
end

puts m(x)
puts e(x)
puts c(x)
