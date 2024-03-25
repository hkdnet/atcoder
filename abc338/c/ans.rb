def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
Q = getis
A = getis
B = getis

ans = 0

0.upto(1_000_000) do |ma|
  mb = 1_000_000
  f = true
  N.times do |i|
    rest = Q[i] - A[i]*ma
    if rest < 0
      f = false
      break
    end
    next if B[i] == 0
    tmp = rest / B[i]
    mb = tmp if tmp < mb
  end

  if f
    tmp = ma + mb
    ans = tmp if ans < tmp
  else
    break
  end
end
puts ans
