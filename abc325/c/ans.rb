def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

H, W = getis

sensors = Hash.new{|h, k|h[k] = {}}
arr = []
id = 0
H.times.map do |h|
  cs = gets.chomp.chars
  cs.each.with_index do |c, w|
    if c == '#'
      sensors[h][w] = id
      arr << [h, w]
      id += 1
    end
  end
end

# p sensors

uf = Array.new(id) {|n| n }
root = ->(a) {
  if uf[a] == a
    a
  else
    uf[a] = root[uf[a]]
  end
}
unite = ->(a, b) {
  uf[root[b]] = root[a]
}

deltas = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
]

arr.each do |h, w|
  neighbor = nil
  id = sensors[h][w]
  # puts "serach neighbor for id = #{id}(#{h}, #{w})"
  deltas.each do |dx, dy|
    x = h + dx
    y = w + dy
    if x < 0 || x >= H || y < 0 || y >= W
      next
    end
    # puts "checking (#{x}, #{y})"
    neighbor = sensors[x][y]
    if neighbor
      # puts "found neighbor #{neighbor} (#{x}, #{y})"
      unite[id, neighbor]
    end
  end
end

arr.size.times do |a|
  root[a]
end
# uf.to_set.each do |h|
#   p h
#   p arr[h]
# end

# (0..H).each do |x|
#   (0..W).each do |y|
#     if sensors[x][y]
#       id = sensors[x][y]
#       r = root[id]
#       c = ('A'.ord - 1 + r).chr
#       print(c)
#     else
#       print '.'
#     end
#   end
#   puts
# end

puts(
  uf.to_set.size
)

__END__
01234  7  0       8 01      8 0 2       0
.#..#..#####..#...#..#####..#...#...###...#####
.#.#...#.......#.#...#......##..#..#...#..#....
.##....#####....#....#####..#.#.#..#......#####
.#.#...#........#....#......#..##..#...#..#....
.#..#..#####....#....#####..#...#...###...#####
