def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
edges = Hash.new{ |h, k| h[k] = {} }
1.upto(N-1) do |i|
  a, b, x = getis
  edges[i][i+1] = a
  edges[i][x] = b
end

dist = Array.new(N+1) { Float::INFINITY }
dist[1] = 0
cur = 1
while cur != N
  c = dist[cur]
  nx = nil
  _, cur = edges[cur].map do |to, d|
    cost = c + d
    dist[to] = cost if cost < dist[to]

    [cost, to]
  end.min
end

puts dist[N]
