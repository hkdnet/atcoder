h = w = m = 3 * 10**5
hb = 10**3
wb = 3 * 10**2

hs = (1..h).to_a.shuffle.take(hb)
ws = (1..w).to_a.shuffle.take(wb)

puts [h, w, m].join(' ')
hs.each do |x|
  ws.each do |y|
    puts [x, y].join(' ')
  end
end

