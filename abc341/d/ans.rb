def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M, K = getis

tmp = 0
idx = 0
loop do
  tmp += 1
  if (tmp % N == 0 && tmp % M != 0) || (tmp % N != 0 && tmp % M == 0)
    idx += 1
    if idx == K
      puts tmp
      break
    end
  end
end
