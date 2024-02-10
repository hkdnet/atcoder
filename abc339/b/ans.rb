def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W, N = getis

b = H.times.map do
  '.' * W
end

d = [-1, 0]
x = 0
y = 0
def tr(d)
  x, y = d
  [y, -x]
end

def tl(d)
  x, y = d
  [-y, x]
end


N.times do
  d =
    if b[x][y] == '.'
      b[x][y] = '#'
      tr(d)
    else
      b[x][y] = '.'
      tl(d)
    end
  dx, dy = d
  x = (x + dx + H) % H
  y = (y + dy + W) % W
end

b.each do |line|
  puts line
end
