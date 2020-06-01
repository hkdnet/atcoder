MAX = 10**18

_ = gets # discard n
as = gets.chomp.split(' ').map(&:to_i)

if as.any? { |e| e == 0 }
  puts "0"
else
  ans = 1
  as.each do |a|
    ans *= a
    if ans > MAX
      break
    end
  end

  if ans > MAX
    puts "-1"
  else
    puts ans
  end
end
