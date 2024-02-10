def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
B = N.times.map do
  gets.chomp.chars
end
dist = {}

p1 = nil
p2 = nil
B.each.with_index do |line, x|
  line.each.with_index do |c, y|
    if c == 'P'
      if p1.nil?
        p1 = [x, y]
      else
        p2 = [x, y]
      end
    end
  end
end

def to_state(p1, p2)
  x1, y1 = p1
  x2, y2 = p2
  [x1, y1, x2, y2].map(&:to_s).join("$")
end

def from_state(s)
  x1, y1, x2, y2 = s.split("$").map(&:to_i)
  [[x1, y1], [x2, y2]]
end

start_at = to_state(p1, p2)
q = []
q << start_at

dist[start_at] = 0

DIR = [
  [1, 0],
  [-1, 0],
  [0, 1],
  [0, -1],
]

def valid?(p)
  x, y = p
  return false if x < 0 || y < 0 || x >= N || y >= N
  B[x][y] == '.'
end

ans = nil

until q.empty?
  s = q.shift
  cnt = dist[s]
  p1, p2 = from_state(s)
  p [p1, p2, cnt]
  if p1 == p2
    ans = cnt
    break
  end
  x1, y1 = p1
  x2, y2 = p2

  DIR.each do |dx, dy|
    nx1 = [x1 + dx, y1 + dy]
    nx1 = p1 unless valid?(nx1)
    nx2 = [x2 + dx, y2 + dy]
    nx2 = p2 unless valid?(nx2)

    nx = to_state(nx1, nx2)
    next if dist[nx]
    dist[nx] = cnt + 1
    q << nx
  end
end

if ans
  puts ans
else
  puts "-1"
end
