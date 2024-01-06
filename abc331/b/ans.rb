def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, S, M, L = getis

ans = S * N

0.upto(N) do |s|
  0.upto(N) do |m|
    cnt = 6 * s + 8 * m
    rest = N - cnt
    l = rest / 12
    l += 1 if rest % 12 != 0

    price = s * S + m * M + l*L
    ans = price if ans > price
    if l == 0
      break
    end
  end
  break if 6 * s > N
end

puts ans
