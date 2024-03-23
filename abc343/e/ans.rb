def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

v1, v2, v3 = getis

def d(x, y, z)
  return 0 if x > 7 || y > 7 || z > 7
  dx = 7 - x
  dy = 7 - y
  dz = 7 - z
  dx * dy * dz
end

def vv(dx, dy, dz)
  if dx > 0 && dy > 0 && dz > 0
    dx * dy * dz
  else
    0
  end
end

def overlap(x1, x2)
  return 7 if x1 == x2
  if x2 > x1
    overlap(x2, x1)
  end
  # x1 < x2
  # x1 ---- x1 + 7
  #    x2 ---- x2 + 7
  l = x1 + 7 - x2
  return 0 if l < 0
  l
end

def overlap3(x2, x3)
  if x2 == x3
    if x2 > 7
      return 0
    else
      return 7 - x2
    end
  end

  if x2 > x3
    overlap3(x3, x2)
  end
  # x2 < x3
  l = x2 + 7 - x3
  return 0 if l < 0
  l
end

def f(v1, v2, v3)
  range = (0..7)
  if v1 + v2 * 2 + v3 * 3 == 1029
    x1 = 0
    y1 = 0
    z1 = 0
    range.each do |x2|
      range.each do |y2|
        range.each do |z2|
          range.each do |x3|
            range.each do |y3|
              range.each do |z3|
                v12 = d(x2, y2, z2)
                v13 = d(x3, y3, z3)
                v23 = vv(overlap(x2, x3), overlap(y2, y3), overlap(z2, z3))
                v123 = vv(overlap3(x2, x3), overlap3(y2, y3), overlap3(z2, z3))

                if v2 == (v12 + v13 + v23 - 3 * v123) && v3 == v123
                  return [x1, y1, z1, x2, y2, z2, x3, y3, z3]
                end
              end
            end
          end
        end
      end
    end
  end

  nil
end

a = f(v1, v2, v3)
if a
  puts "Yes"
  puts a.join(" ")
else
  puts "No"
end
