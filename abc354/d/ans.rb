def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

a, b, c, d = getis.map { |e| e + (10**9 * 4) }
A = a % 4
B = b % 4
C = c - (a-A)
D = d - (b-B)

arr = [
  [1, 2, 1, 0],
  [2, 1, 0, 1],
  [1, 2, 1, 0],
  [2, 1, 0, 1], # y = [0, 1)
]
ARR = arr.reverse
SUM = ARR.sum { |e| e.sum }

def f(x, y)
  dx = x % 4
  dy = y % 4
  lx = (x - dx)/4
  ly = (y - dy)/4
  ret = lx*ly*SUM
  ret += lx * ARR.take(dy).sum(&:sum)
  ret += ly * ARR.sum { |e| e.take(dx).sum }
  ret += ARR.take(dy).sum { |e| e.take(dx).sum }
  ret
end

# STDERR.puts [A, B, C, D].inspect

ans = f(C, D) - f(C, B) - f(A, D) + f(A, B)

puts ans
