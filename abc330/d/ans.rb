def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = N.times.map do
  gets.chomp
end

x_count = N.times.map do |x|
  cnt = 0
  S[x].chars.each do |c|
    cnt += 1 if c == 'o'
  end
  cnt
end
y_count = N.times.map do |y|
  cnt = 0
  N.times do |x|
    cnt += 1 if S[x][y] == 'o'
  end
  cnt
end

ans = 0

N.times do |x|
  N.times do |y|
    next if S[x][y] != 'o'

    xc = x_count[x]
    yc = y_count[y]
    if xc > 1 && yc > 1
      ans += (xc - 1) * (yc - 1)
    end
  end
end

puts ans
