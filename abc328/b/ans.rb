def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
D = getis

c = 0
D.each.with_index(1) do |d, m|
  mm = m.to_s.chars
  next unless mm.all? { |e| e == mm[0] }

  dc = mm[0]
  dd = dc
  while dd.to_i <= d
    c += 1
    dd += dc
  end
end

puts c
