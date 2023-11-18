def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

gets
a = getis
a.sort!
puts(a.filter { |e| e != a[-1] }.last)
