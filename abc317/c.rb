N, M = gets.chomp.split(' ').map(&:to_i)
edges = Hash.new {|h, k| h[k] = {} }
M.times do
  a, b, c = gets.chomp.split(' ').map(&:to_i)
  edges[a][b] = c
  edges[b][a] = c
end

used = []
ans = 0

dfs = ->(cur, d) {
  used[cur] = true
  edges[cur].each do |to, dd|
    if !used[to]
      nd = d + dd
      ans = nd if ans < nd
      dfs.call(to, d+dd)
    end
  end
  used[cur] = false
}

N.times.map do |i|
  dfs.call(i + 1, 0)
end

puts ans
