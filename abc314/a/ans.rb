def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


s = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679"

N = geti
puts(s[0..N+1])