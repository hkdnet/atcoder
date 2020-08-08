x, k, d = gets.chomp.split(' ').map(&:to_i)

if x < 0
  x = -x
end

if x / d > k
  x -= k * d
  puts x
else
  s = x / d
  x -= d * s
  k -= s

  if k % 2 == 0
    puts x
  else
    puts (x - d).abs
  end
end
