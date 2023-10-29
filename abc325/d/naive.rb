def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
max = 0
products = N.times.map do
  t, d = getis
  t1 = t + d
  max = t1 if t1 < max
  [t, t1]
end

ans = 0
(0...N).to_a.permutation(N) do |pattern|
  cur = 0
  tmp = 0
  pattern.each do |idx|
    a, b = products[idx]
    if cur <= b
      cur = a if cur < a
      tmp += 1
      cur += 1
    end
  end

  ans = tmp if ans < tmp
end

puts ans
