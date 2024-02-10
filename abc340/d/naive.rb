def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
edges = []
1.upto(N-1) do |i|
  edges << getis
end

dist = Array.new(N+1) { Float::INFINITY }

dist[1] = 0

(N-1).times do
  edges.each.with_index(1) do |(a, b, x), i|
    next if dist[i] == Float::INFINITY
    if dist[i+1] > dist[i] + a
      dist[i+1] = dist[i] + a
    end
    if dist[x] > dist[i] + b
      dist[x] = dist[i] + b
    end
  end
end

puts dist[N]
