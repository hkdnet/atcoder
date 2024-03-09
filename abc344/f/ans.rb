def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti

P = N.times.map do
  getis
end
R = N.times.map do
  getis
end
D = (N-1).times.map do
  getis
end

arr = [{ turn: 0, money: 0, max: P[0][0] }]
(N-1).times do |i|
  prev = arr[i]
  turn = prev[:turn]
  money, turn_delta = move(money: prev[:money], max: prev[:max], cost: R[0][i])
  turn += turn_delta
  max = prev[:max]
  if P[0][i+1] > max
    max = P[0][i+1]
  end
  h = { turn:, money:, max: }
  arr[i+1] = h
end

def maxv(a, b)
  if a > b
    a
  else
    b
  end
end

def move(money:, max:, cost:)
  rest = money - cost
  turn = 1
  if rest < 0
    mul = (-rest) / max
    rest += mul * max
    turn += mul
    if rest < 0
      rest += max
      turn += 1
    end
  end
  [rest, turn]
end

(N-1).times do |x| # from x to x+1
  tmp = []
  begin
    m, d = move(money: arr[0][:money], max: arr[0][:max], cost: D[x][0])
    tmp << { turn: arr[0][:turn] + d, money: m, max: maxv(arr[0][:max], P[x+1][0]) }
  end

  (N-1).times do |y|
    y = y + 1

  end

  arr = tmp
end

ans = arr[N-1][N-1][:turn]
puts ans
