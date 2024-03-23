def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

S = gets.chomp
N = S.size

h = {}

N.times do |i|
  (i+1).upto(N-1) do |j|
    s = S.dup
    tmp = s[i]
    s[i] = s[j]
    s[j] = tmp
    h[s] = true
  end
end

STDERR.puts h.keys
puts h.size
