def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, Q = getis
A = getis
bucket_size = Math.sqrt(N).ceil

