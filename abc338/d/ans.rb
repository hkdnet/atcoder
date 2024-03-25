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
# 寄与度 i: iから i+1 の橋が落ちてたときにロスになる値の合計。imos
arr = Array.new(N+1) { 0 }
shortest = 0
X.each_cons(2) do |x1, x2|
  if x2 < x1
    x1, x2 = x2, x1
  end
  c, ant = dist(x1, x2)

  shortest += [c, ant].min

  # STDERR.puts("区間", [x1, x2].inspect)
  # STDERR.puts([c,ant].inspect)
  if c < ant
    d = ant - c
    # STDERR.puts("add きよど #{d} to x1-x2")
    arr[x1] += d
    arr[x2] -= d
  elsif ant < c
    d = c - ant
    # STDERR.puts("add きよど #{d} to 0-x1, x2-n")
    arr[0] += d
    arr[x1] -= d
    arr[x2] += d
    arr[N] -= d
  end

  # STDERR.puts arr.inspect
end


v = []
arr.take(N).each do |a|
  last = v.last || 0
  v << last + a
end
# STDERR.puts v.inspect
# STDERR.puts "shortest = #{shortest} min = #{v.min}"
ans = shortest + v.min
puts ans
