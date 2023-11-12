_ = gets
as = gets.chomp.split(' ').map(&:to_i)
as.sort!
tmp = 0
loop do
  if as[tmp +1] != as[tmp] + 1
    puts as[tmp] + 1
    break
  end
  tmp += 1
end
