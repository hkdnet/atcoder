def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N, Q = getis

parts = N.downto(1).map do |i|
  [i, 0]
end

D = {
  "L" => [-1, 0],
  "R" => [ 1, 0],
  "U" => [ 0, 1],
  "D" => [ 0,-1],
}

Q.times do
  ty, a = gets.chomp.split(" ")
  if ty == "1"
    d = D.fetch(a)
    x, y = parts.last
    parts << [x + d[0], y + d[1]]
  else
    idx = parts.size - a.to_i
    puts parts[-1*a.to_i].join(" ")
  end
end
