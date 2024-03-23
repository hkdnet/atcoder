def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp
Q = geti
az = ('a'..'z').to_a
s = az.join("")

Q.times do
  c, d = gets.chomp.split(" ", 2)
  s.gsub!(c, d)
end
h = az.zip(s.chars).to_h

ans = S.chars.map { |c| h[c] }.join("")
puts ans
