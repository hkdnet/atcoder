def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
s = S

loop do
  ss = s.gsub("ABC", "")
  if s == ss
    break
  end
  s = ss
end

puts s
