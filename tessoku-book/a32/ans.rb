def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, A, B = getis
g = [] # true で勝ち, false で負け
(N+1).times do |i|
  s =
    if i >= A && !g[i-A] # A を選んだら負けに遷移できる
      true
    elsif i >= B && !g[i-B] # B を選んだら負けに遷移できる
      true
    else
      false
    end
  g[i] = s
end

if g[N]
  puts "First"
else
  puts "Second"
end
