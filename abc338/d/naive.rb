def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
# 0-origin
X = getis.map {|e| e - 1 }

def dist(s, d)
  # s < d
  [d - s, s + N - d]
end

ans = Float::INFINITY
N.times do |i|
  tmp = 0
  X.each_cons(2) do |x1, x2|
    if x2 < x1
      x1, x2 = x2, x1
    end
    c, ant = dist(x1, x2)

    if x1 <= i && i < x2
      tmp += ant
    else
      tmp += c
    end
  end
  ans = tmp if tmp < ans
end

puts ans
