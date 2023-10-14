def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
a = N.times.map do
  f, s = getis
  [s, f]
end.sort_by{|e| -e[0]}

if a[0][1] != a[1][1]
  puts a[0][0] + a[1][0]
else
  max = a[0][0] + a[1][0]/2
  base = a[0][0]
  base_f = a[0][1]
  a.each do |s, f|
    next if base_f == f

    tmp = base + s
    if tmp > max
      max = tmp
    end
    break
  end

  puts max
end
