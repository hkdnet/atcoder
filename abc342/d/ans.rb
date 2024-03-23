def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

MAX = 2*10**5
osak = Array.new(MAX + 1) { |n| n }
2.upto(MAX) do |k|
  next unless osak[k] == k
  idx = k
  while idx < osak.size
    idx += k
    osak[idx] = k if osak[idx] == idx
  end
end

N = geti
A = getis
h = Hash.new do |h, k|
  h[k] = 0
end

z = 0
A.each do |i|
  if i == 0
    z += 1
    next
  end

  #puts i
  if i < 1
    h[1] += 1
    next
  end

  tmp = 1
  while i >= 2
    divider = osak[i]
    if i == divider
      tmp *= divider
      break
    end

    if i % (divider * divider) == 0
      i /= divider
      i /= divider
    else
      i /= divider
      tmp *= divider
    end
  end
  h[tmp] += 1
  #p h
end
# p h
ans = 0
h.each do |_, n|
  ans += n * (n - 1) / 2
end

if z > 0
  nz = N - z
  ans += z * nz
  ans += z * (z - 1) / 2
end

puts ans

