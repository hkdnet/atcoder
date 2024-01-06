def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti

cur = [0, 0]
d = [0, 1]
m = N.times.map { Array.new(N) { 'T' } }
1.upto(N*N-1) do |i|
  x, y = cur
  m[x][y] = i
  xx = x + d[0]
  yy = y + d[1]
  if 0 <= xx && xx < N && 0 <= yy && yy < N && m[xx][yy] == 'T'
    cur = [xx, yy]
  else
    dx, dy = d
    d = [dy, -dx]
    xx = x + d[0]
    yy = y + d[1]
    cur = [xx, yy]
  end
end

m.each do |arr|
  puts arr.join(" ")
end
