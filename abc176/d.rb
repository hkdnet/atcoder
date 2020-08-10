def g
  gets.chomp.split(' ').map(&:to_i)
end
h, w = g
start = g
gx, gy = g
# 1-idx
m = h.times.with_object(Hash.new{|h,k|h[k] = {}}) do |x, d|
  d[x+1] = {}
  gets.chomp.chars.each.with_index(1) do |c, y|
    d[x+1][y] = c
  end
end

q = []
used = Hash.new {|h, k| h[k] = {} }
jump = []
jump << start
jump_count = -1
while !jump.empty?
  jx, jy = jump.pop
  next if used[jx][jy]

  # puts "jump to (#{jx}, #{jy})"

  jump_count += 1
  q << [jx, jy]
  used[jx][jy] = true

  while !q.empty?
    # h, w
    x, y = q.pop
    # puts "step to (#{x}, #{y})"
    if x == gx && y == gy
      puts jump_count
      exit 0
    end
    [
      [x-1, y],
      [x+1, y],
      [x, y-1],
      [x, y+1],
    ].each do |xx, yy|
      if m[xx][yy] == '.'
        if !used[xx][yy]
          q << [xx, yy]
          used[xx][yy] = true
        end
      end
    end

    (x-2).upto(x+2) do |xx|
      (y-2).upto(y+2) do |yy|
        if m[xx][yy] == '.'
          jump << [xx, yy]
        end
      end
    end
  end
end

puts -1
