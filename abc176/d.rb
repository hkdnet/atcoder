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
jumps = []
jumps[0] = []
jumps[0] << [start[0], start[1]]

loop do
  jumps.each_with_index do |jump, jump_count|
    while !jump.empty?
      jx, jy = jump.pop
      next if used[jx][jy]

      # puts "jump to (#{jx}, #{jy})"

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
              jumps[jump_count+1] ||= []
              jumps[jump_count+1] <<  [xx, yy]
            end
          end
        end
      end
    end
  end

  if jumps.all?(&:empty?)
    puts -1
    exit 0
  end
end

