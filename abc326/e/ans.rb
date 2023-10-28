def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
as = getis
dp = Array.new(N+1) { 0 }
