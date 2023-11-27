def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

D = geti
ans = nil

1.upto(D) do |a|
  1.upto(D) do |b|
    tmp = (a**2 + b ** 2 - D).abs

    ans = tmp if ans.nil? || ans > tmp
  end
end

puts ans
