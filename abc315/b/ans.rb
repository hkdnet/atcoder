def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

M = geti
ds = getis
s = ds.sum
h = (s+1)/2
idx = 0
loop do
  tmp = h - ds[idx]
  if tmp > 0
    idx += 1
    h = tmp
  else
    puts "#{idx+1} #{h}"
    break
  end
end
