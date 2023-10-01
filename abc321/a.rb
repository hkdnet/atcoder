def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

f = true
geti.to_s.chars.each_cons(2) do |a, b|
  a = a.to_i
  b = b.to_i
  if a <= b
    f = false
    break
  end
end

puts(f ? "Yes" : "No")
