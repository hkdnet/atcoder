def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
edges = Hash.new{|h, k| h[k] = {}}
rev = Hash.new{|h, k| h[k] = {}}
leaf = {}
1.upto(N) do |i|
  c, *ps = getis

  leaf[i] = true if c == 0

  ps.each do |e|
    edges[i][e] = true
    rev[e][i] = true
  end
end

required = {1 => true}
q = [1]

while !q.empty?
  id = q.pop
  nexts = edges[id].keys
  nexts.each do |nx|
    if required[nx]
      # do nothing
    else
      required[nx] = true
      q << nx
    end
  end
end

# トポロジカルソート topological sort
ans = []
q = leaf.keys.select {|e| required[e] }
counts = edges.to_h {|id, arr| [id, arr.select { |e| required[e] }.size] }

while !q.empty?
  id = q.pop
  ans << id
  rev[id].keys.each do |to|
    counts[to] -= 1
    if counts[to] == 0
      q << to
    end
  end
end

puts ans.reject{|e| e == 1 }.join(" ")

