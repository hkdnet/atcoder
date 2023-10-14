def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

n, T = gets.chomp.split(" ")
N = n.to_i
s = N.times.map {gets.chomp}

hs = s.map do |ss|
  idx = -1
  ss.chars.each do |c|
    if c == T[idx + 1]
      idx += 1
    end
  end
  idx
end
ts = s.map do |ss|
  idx = T.size
  ss.chars.reverse_each do |c|
    if c == T[idx - 1]
      idx -= 1
    end
  end
  idx
end
ts.sort!
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

ans = 0

cond = ->(h, t) {
  t <= h + 1
}

hs.each do |h|
  if !cond[h, ts[0]]
    next
  end
  i = bin_search(0, ts.size) { |idx| cond[h, ts[idx]] }
  ans += i + 1
end

puts ans

