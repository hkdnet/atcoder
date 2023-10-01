def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

k = geti
digits = 1.upto(9).map(&:to_s)
if k <= 9
  puts(k)
else
  k -= 9
  prev = %w[0 1 2 3 4 5 6 7 8 9]
  loop do
    tmp = []
    digits.each do |nx|
      tmp += prev.filter_map do |e|
        "#{nx}#{e}" if nx > e[0]
      end
    end
    prev = tmp
    if prev[k-1]
      break
    else
      k -= prev.size
    end
  end
  puts prev[k-1]
end
