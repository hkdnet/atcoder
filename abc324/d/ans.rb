def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp
MAX = 10**(S.size)

arr = []
sorted = S.chars.sort
format_str = "%0#{S.size}d"
ans = 0
1.upto(MAX) do |n|
  n = n ** 2
  break if n > MAX

  s = format_str % n
  if s.chars.sort == sorted
    ans += 1
  end
end

puts ans
