def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
D = getis
short, long = 2.times.map { getis }.sort_by { |e| e[0] }
sl, sc, sk = short
ll, lc, lk = long


