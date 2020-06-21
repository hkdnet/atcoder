n = gets.chomp.to_i

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
1.upto(100) do |n|
  puts s(n)
end


