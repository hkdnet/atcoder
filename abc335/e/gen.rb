# @return [Integer] an integer of [l, r)
def randi(l, r)
  rand(r-l) + l
end

# arr.shuffle
# arr.sample
# arr.sample(n)

N = 10
edges = Hash.new { |h, k| h[k] = {} }
a = 1.upto(N).to_a
a.shuffle!
a.each_cons(2) do |u, v|
  u, v = v, u if u > v
  edges[u][v] = true
end

A = N.times.map { randi(1, 20) }

add = 5
while add > 0
  u = randi(1, N)
  v = randi(1, N)
  next if u == v
  u, v = v, u if u > v
  next if edges[u][v]

  edges[u][v] = true
  add -= 1
end

M = edges.map { |k, v| v.size }.sum

puts "#{N} #{M}"
puts A.join(" ")
edges.each do |u, arr|
  arr.each do |v, _|
    puts "#{u} #{v}"
  end
end
