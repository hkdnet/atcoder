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
rect = ->(a, b, c, d) {
  acc[c+1][d+1] - acc[a][d+1] - acc[b][c+1] + acc[a][b]
}

solve = ->(a, b, c, d) {
  if a >= N
    orig = a
    a = orig % N
    c -= (orig - a)
    next solve[a, b, c, d]
  end
  if b >= N
    orig = b
    b = orig % N
    d -= (orig - b)
    next solve[a, b, c, d]
  end

  if a < 0 || b < 0 || c < 0 || d < 0
    raise "#{[a, b, c, d].join(" ")}"
  end

  na = next_n(a)
  nb = next_n(b)
  ret =
    if na == next_n(c)
      if nb == next_n(d)
        # またがない
        rect[a, b, c, d]
      else
        w = (prev_n(d) - nb)/N
        # 横にだけまたぐ
        # l と r と間
        l = solve[a, b, c, N - 1]
        r = solve[a, 0, c, d % N]
        im = 0
        im = solve[a, 0, c, N - 1] * w if w > 0
        l + r + im
      end
    else
      h = (prev_n(c) - na)/N
      # 縦にはまたぐ
      t = solve[a, b, N - 1, d]
      bot = solve[0, b, c % N, d]
      im = 0
      im = solve[0, b, N - 1, d] * h if h > 0
      t + bot + im
    end
  # p [a, b, c, d, ret]
  ret
}

Q.times do
  a, b, c, d = getis
  puts solve.call(a, b, c, d)
end
