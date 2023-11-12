N, M = gets.chomp.split(' ').map(&:to_i)
edges = Hash.new {|h, k| h[k] = {} }
M.times do
  a, b, c = gets.chomp.split(' ').map(&:to_i)
  a = 1 << (a-1)
  b = 1 << (b-1)
  edges[a][b] = c
  edges[b][a] = c
end

q = []
ans = 0
cnt = 0
N.times do |i|
  cur = 1 << i
  q << [cur, 0, cur]
end
while !q.empty?
  cnt += 1
  cur, d, s = q.pop
  edges[cur].each do |to, delta|
    next if (s & to) != 0

    nd = d + delta
    ans = nd if ans < nd
    q << [to, nd, s | to]
  end
end

puts ans
puts cnt
