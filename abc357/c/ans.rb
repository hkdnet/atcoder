def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

N = geti
def f(k)
  if k == 0
    '#'
  else
    s = f(k-1)
    lines = s.split("\n")
    ret = lines.map { |line| line * 3 }
    ret += lines.map { |line| line * 3 }
    sq = lines.size
    sq.times do |i|
      sq.times do |j|
        ret[-i-1][sq+j] = '.'
      end
    end
    ret += lines.map { |line| line * 3 }
    ret.join("\n")
  end
end

puts f(N)
