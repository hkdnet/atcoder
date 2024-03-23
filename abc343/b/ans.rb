def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
m = N.times.map do
  getis
end

N.times do |i|
  arr = m[i]
  ps = arr.zip(1..).filter_map do |(c, i)|
    if c == 1
      i
    else
      nil
    end
  end
  puts ps.join(" ")
end
