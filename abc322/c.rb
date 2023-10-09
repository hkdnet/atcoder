def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
aa = getis

idx = 0
N.times do |i|
  nx = aa[idx] - 1
  puts(nx - i)
  if nx == i
    idx += 1
  end
end
