ps = 3.times.map do
  lines = 4.times.map { gets.chomp }.reject {|e| e == '....' }
  while lines.all?{|e| e[0] == '.' }
    lines = lines.map {|e| e[1..] }
  end

  points = []
  x = 0
  y = lines[x].index('#')

  lines.each.with_index do |line, i|
    line.split("").each.with_index do |c, j|
      if c == "#"
        points << [i-x, j-y]
      end
    end
  end

  points
end

def rotate(points)
  points.map do |x, y|
    [y, -x]
  end
end

patterns = ps.map do |pp|
  a = []
  4.times do |i|
    arr = pp
    i.times { arr = rotate(arr) }
    4.times do |x|
      4.times do |y|
        mapped = arr.map { |point| [x + point.first, y + point.last] }
        is_ok = mapped.all? do |x, y|
          0 <= x && x <= 3 && 0 <= y && y <= 3
        end
        a << mapped if is_ok
      end
    end
  end

  a
end

possible = false
patterns[0].each do |pa|
  patterns[1].each do |pb|
    patterns[2].each do |pc|
      ps = pa + pb + pc
      ss = ps.map {|arr| arr.join("$") }.uniq
      if ps.size == ss.size && ss.size == 16
        possible = true
      end
    end
  end
end

puts(possible ? "Yes" : "No")
