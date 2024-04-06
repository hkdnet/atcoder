def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
points = N.times.map do |i|
  x, y = getis
  [i + 1, x, y]
end

h = Hash.new do |h, k|
  h[k] = [0, 0]
end

points.combination(2) do |p1, p2|
  id1, x1, y1 = p1
  id2, x2, y2 = p2
  d = (x1-x2)**2 + (y1-y2)**2
  h[id1] = [d, id2] if h[id1][0] < d || (h[id1][0] == d && h[id1][1] > id2)
  h[id2] = [d, id1] if h[id2][0] < d || (h[id2][0] == d && h[id2][1] > id1)
end

N.times do |i|
  _, id = h[i+1]
  puts id
end
