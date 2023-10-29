def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N, M = getis

ps = getis
ps.sort!

l = 0
r = 1
ans = 1

while r < N
  x = ps[l]
  y = x + M
  while r < N
    if ps[r] < y
      r += 1
    else
      break
    end
  end

  tmp = r - l

  ans = tmp if ans < tmp
  l += 1
end

puts(ans)
