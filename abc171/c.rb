n = gets.chomp.to_i

def ss(n)
  arr = []

  while n != 0 do
    tmp = n % 26
    if tmp == 0
      arr << 'z'
      n -= 26
    else
      arr << ('a'.ord + tmp - 1).chr
    end
    n /= 26
  end

  arr.reverse().join()
end

def s(n)
  s = n.to_s(26)
  cs = s.chars.map do |c|
    if c < 'a'
      ('a'.ord + c.to_i - 1).chr
    else
      (c.ord + 9).chr
    end
  end
  cs.join('')
end

# puts s(n)
puts ss(n)
# 1.upto(100) do |n|
#   puts ss(n)
# end


