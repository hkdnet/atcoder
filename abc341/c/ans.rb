def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W, N = getis
T = gets.chomp.chars
B = H.times.flat_map do
  gets.chomp.chars
end

def f(x, y)
  return false unless B[W * x + y] == '.'

  T.each do |d|
    case d
    when "L"
      y -= 1
    when "R"
      y += 1
    when "U"
      x -= 1
    when "D"
      x +=1
    end
    if x < 0 || y < 0
      return false
    end

    if B[W * x + y] != '.'
      return false
    end
  end

  true
end

ans = 0
H.times do |x|
  W.times do |y|
    if f(x, y)
      ans += 1
    end
  end
end

puts ans
