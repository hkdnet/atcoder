def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

xa, ya, xb, yb, xc, yc = getis

bc = []
if yb > yc
  bc << :N
elsif yb < yc
  bc << :S
end
if xb > xc
  bc << :W
elsif xb < xc
  bc << :E
end

ab = []
if ya > yb
  ab << :N
elsif ya < yb
  ab << :S
end
if xa > xb
  ab << :W
elsif xa < xb
  ab << :E
end

diff = (xb-xa).abs + (yb- ya).abs + (xc-xb).abs + (yc-yb).abs

if (ab & bc).first
  puts(diff + 1)
else
  delta = ab.flat_map do |d1|
    bc.map do |d2|
      if (d1 == :S && d2 == :N) &&
        (d1 == :N && d2 == :S) &&
        (d1 == :E && d2 == :W) &&
        (d1 == :W && d2 == :E)
        2
      else
        1
      end
    end
  end.min
  puts(diff + delta + 1)
end
