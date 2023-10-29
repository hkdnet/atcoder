def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


N, A, B, C = getis
m = N.times.map do
  getis
end

shortest = ->(beg, calc) {
  ret = Array.new(N) { -1 }
  N.times do |i|
    ret[i] = beg == i ? 0 : calc[m[beg][i]]
  end
  q = N.times.map {|i|[i, true]}.to_h
  while !q.empty?
    i = q.keys.first
    q.delete(i)
    N.times do |j|
      nx = ret[i] + calc[m[i][j]]
      if ret[j] > nx
        ret[j] = nx
        q[j] = true
      end
    end
  end

  ret
}
h = shortest[0, ->(d){ d * A}]
t = shortest[N-1, ->(d) { d*B+C }]
ans = N.times.map do |i|
  h[i] + t[i]
end.min

puts ans
