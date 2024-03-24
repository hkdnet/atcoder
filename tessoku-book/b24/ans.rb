def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti

h = Hash.new do |h, k|
  h[k] = []
end
N.times.each do
  x, y = getis
  h[x] << y
end
h.values.each(&:sort!)

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

# rev[i]: i 個重ねたときの最小の y
rev = []
rev[0] = 0
ans = 0
h.keys.sort.each do |x|
  ys = h[x]
  ns = {}
  ys.each do |y|
    max_cnt = binary_search(0, N) do |idx|
      next false if rev[idx].nil?
      rev[idx] < y
    end
    cnt = max_cnt + 1
    ns[y] = cnt
    ans = cnt if ans < cnt
  end

  ns.each do |y, cnt|
    rev[cnt] = y if rev[cnt].nil? || y < rev[cnt]
  end
end

puts ans
