def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, H, W = getis
tiles = N.times.map do
  getis
end

def solve(tiles)
  (1<<N).times do |rev_state|
    rotated_tiles = []
    tiles.each.with_index do |(a, b), i|
      if rev_state&(1<<i) > 0
        rotated_tiles << [b, a]
      else
        rotated_tiles << [a, b]
      end
    end
    next if rotated_tiles.size != tiles.size

    if sub_solve(rotated_tiles)
      return true
    end
  end

  false
end

def sub_solve(tiles)
  b = Hash.new do |h, k|
    b[h] = k
  end

  tile_index = 0

  H.times do |h|
    W.times do |w|
      next unless b[h][w].nil?

      x, y = tiles[tile_index]
      return false if x.nil?

      if h + x >= H || w + y >= W
        return false
      end
      x.times do |dx|
        y.times do |dy|
          return false if b[h+x][h+y]
          b[h+x][h+y] = true
        end
      end

      tile_index += 1
    end
  end

  return false if tile_index != tiles.size

  H.times do |h|
    W.times do |w|
      unless b[h][w]
        return false
      end
    end
  end
end

f = tiles.permutation.any? do |tiles|
  solve(tiles)
end

if f
  puts "Yes"
else
  puts "No"
end
