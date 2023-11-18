def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, Q = getis
cc = getis
v = Array.new(N + 1) { {} }
cc.each.with_index(1) do |c, i|
  v[i][c] = true
end

ids = Hash.new{|h, k| h[k] = k }

Q.times do
  a, b = getis
  if v[ids[a]].size > v[ids[b]].size
    swap = true
    tmp = a
    a = b
    b = tmp
  end
  v[ids[a]].keys.each {|k| v[ids[b]][k] = true }
  v[ids[a]] = {}

  puts v[ids[b]].size

  if swap
    tmp = ids[a]
    ids[a] = ids[b]
    ids[b] = tmp
  end
end
