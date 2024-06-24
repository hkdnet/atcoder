def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }


def bin_search(l, r)
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


N = geti
A = getis
C = Array.new(10**6+1) { 0 }
A.each do |i|
  C[i] += 1
end
1.upto(10**6) do |i|
  C[i] += C[i-1]
end
ans = 0

1.upto(10**6) do |i|
  tmp = i
  cnt = C[i]-C[i-1]
  next if cnt == 0

  while tmp < 10**6+1
    cur = tmp/i
    idx = tmp+i-1
    idx = C.size - 1 if idx >= C.size
    c = C[idx] - C[tmp-1]
    # p [i, idx, tmp, c, cur, cnt] if c != 0
    ans += c * cur * cnt
    tmp += i
  end
  ans -= cnt*(cnt+1)/2
end

puts ans
