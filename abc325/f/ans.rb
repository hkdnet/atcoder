def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
D = getis
s, l = 2.times.map { getis }.sort_by { |e| e[0] }

