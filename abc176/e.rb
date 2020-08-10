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


