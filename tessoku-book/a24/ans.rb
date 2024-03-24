def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis

def binary_search(l, r)
  loop do
    idx = (l + r)/2
    return idx if idx == l || idx == r
    f = yield idx
    if f
      l = idx
    else
      r = idx
    end
  end
end

# rev[i] = n: 長さが i のときの最後の要素の最小値
rev = Array.new(N+1) { Float::INFINITY }
rev[0] = 0
ans = 0
A.each do |a|
  max = binary_search(0, N) do |idx|
    rev[idx] < a
  end
  len = max + 1
  rev[len] = a if a < rev[len]
  ans = len if ans < len
end

puts ans
