def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis
M = geti
B = getis
L = geti
C = getis
Q = geti
X = getis

h = {}
A.each do |a|
  B.each do |b|
    C.each do |c|
      h[a + b + c] = true
    end
  end
end

X.each do |x|
  if h.key?(x)
    puts "Yes"
  else
    puts "No"
  end
end
