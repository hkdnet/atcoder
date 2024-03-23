def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


N = geti
A = getis
ans = 0
N.times do |i|
  (i+1).upto(N-1)do |j|
    a = A[i] * A[j]
    s = Math.sqrt(a).to_i
    if s >= 0 && s * s == a
      ans +=1
    end
  end
end
puts ans
