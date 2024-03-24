def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
A = getis

ans = 0

(1<<N).times do |s|
  arr = A.each.with_index.filter_map do |a, i|
    if s & (1<<i) > 0
      a
    else
      nil
    end
  end
  if arr.each_cons(2).all? { |a, b| a < b }
    tmp = arr.size
    ans = tmp if ans < tmp
  end
end

puts ans
