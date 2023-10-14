def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W = getis
c = H.times.map { gets.chomp.split("") }
count_h = ->(idx) {
  h = Hash.new{|h, k| h[k] = 0}
  c[idx].each do |cc|
    h[cc] += 1
  end
  h
}
count_w = ->(idx) {
  h = Hash.new{|h, k| h[k] = 0}
  H.times do |h|
    cc = c[h][idx]
    h[cc] += 1
  end
  h
}
hw_idx = ->(type, i) {
  type == :H ? i : i + H
}

rest = []
H.each do |h|
  i = hw_idx[:H, h]
  rest[i] = count_h[h]
end
W.each do |w|
  i = hw_idx[:W, w]
  rest[i] = count_w[w]
end

markable = ->(h) {
  h.size == 1 && h.values.first.size > 1
}



