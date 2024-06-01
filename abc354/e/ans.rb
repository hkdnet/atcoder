def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N = geti
HS = {}
TS = {}
N.times.map do |i|
  h, t = getis
  HS[h] ||= []
  HS[h] << i
  TS[t] ||= []
  TS[t] << i
end

MEMO = {}
def f(used, turn)
  return MEMO[used] if MEMO.key?(used)

  HS.each do |_, arr|
    rest = arr.filter do |e|
      used & (1 << e) == 0
    end

    rest.combination(2) do |a, b|
      nx = used | (1 <<a) | (1<<b)
      MEMO[nx] = f(nx, !turn)
      if MEMO[nx] == turn
        return turn
      end
    end
  end
  TS.each do |_, arr|
    rest = arr.filter do |e|
      used & (1 << e) == 0
    end

    rest.combination(2) do |a, b|
      nx = used | (1 <<a) | (1<<b)
      MEMO[nx] = f(nx, !turn)
      if MEMO[nx] == turn
        return turn
      end
    end
  end

  MEMO[used] = !turn
  !turn
end

if f(0, true)
  puts "Takahashi"
else
  puts "Aoki"
end
