def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
boxes = N.times.map do
  getis
end
ans = 0
(1<<N).times.map do |s|
  b = boxes.each.with_index.filter do |_, i|
    s & (1<<i) > 0
  end.map(&:first)
  b.permutation do |bs|
    f = bs.each_cons(2).all? do |(x1, y1), (x2, y2)|
      x1 < x2 && y1 < y2
    end
    if f
      tmp = b.size
      if ans < tmp
        ans = tmp
      end
      break
    end
  end
end
puts ans
