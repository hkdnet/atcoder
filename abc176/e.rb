h, w, m = gets.chomp.split(' ').map(&:to_i)
cells = []
hc = {}
wc = {}
m.times.map do
  h, w = gets.chomp.split(' ').map(&:to_i)
  cells[h] ||= []
  cells[h][w] = true
  hc[h] ||= 0
  hc[h] += 1
  wc[w] ||= 0
  wc[w] += 1
end

hc = hc.map.sort_by {|_, v| -v }
wc = wc.map.sort_by {|_, v| -v }
hc.select! { |k, v| v == hc[0][1] }
wc.select! { |k, v| v == wc[0][1] }
hs = hc.map(&:first)
ws = wc.map(&:first)

found = hs.any? do |x|
  ws.any? do |y|
    !cells[x][y]
  end
end

cnt = hc[0][1] + wc[0][1]
unless found
  cnt -= 1
end

puts cnt
