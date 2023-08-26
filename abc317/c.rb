N, M = gets.chomp.split(' ').map(&:to_i)
edges = Hash.new {|h, k| h[k] = {} }
M.times do
  a, b, c = gets.chomp.split(' ').map(&:to_i)
  a = 1 << (a - 1)
  b = 1 << (b - 1)
  edges[a][b] = c
  edges[b][a] = c
end


visit = ->(i, current, d) {
  return d if i == 0

  tmp = -1

  edges[current].each do |to, delta|
    next if i & to == 0

    ni = i & ~to
    nd = visit.call(ni, to, d + delta)
    if tmp < nd
      tmp = nd
    end
  end

  tmp
}

max_len = 0

(2**N).times do |i|
  c = 1
  loop do
    if c > i
      break
    end

    ni = i & ~c
    d = visit.call(ni, c, 0)
    max_len = d if d > max_len

    c = c << 1
  end
end

puts max_len
