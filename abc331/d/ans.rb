def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


N, Q = getis
P = N.times.map do
  gets.chomp.chars
end

acc = Array.new(N+1) { Array.new(N+1) { 0 } }
P.each.with_index do |cs, i|
  cs.each.with_index do |c, j|
    acc[i+1][j+1] = acc[i+1][j] + acc[i][j+1] - acc[i][j]
    acc[i+1][j+1] += 1 if c == 'B'
  end
end

# acc.each do |arr|
#   puts arr.join(" ")
# end

def prev_n(x)
  next_n(x) - N
end

def next_n(x)
  if x % N == 0
    x + N
  else
    N * (x/N + 1)
  end
end
rect = ->(a, b) {
  ma = a % N
  mb = b % N
  h = a / N
  w = b / N
  h * w * acc[N][N] + acc[ma][N] * w + acc[N][mb] * h + acc[ma][mb]
}

solve = ->(a, b, c, d) {
  rect[c, d] - rect[a, d] - rect[c, b] + rect[a, b]
}

Q.times do
  a, b, c, d = getis
  puts solve.call(a, b, c, d)
end
