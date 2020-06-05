s = gets.chomp
n = s.size
ss = s[0...n/2]
sss = s[(n/2+1)...n]

if s == s.reverse && ss == ss.reverse && sss == sss.reverse
  puts "Yes"
else
  puts "No"
end
