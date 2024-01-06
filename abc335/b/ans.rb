def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
(0..N).each do |a|
  (0..N).each do |b|
    (0..N).each do |c|
      if a + b + c <= N
        puts [a, b, c].join(" ")
      end
    end
  end
end
