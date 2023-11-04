def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

b = 9.times.map do
  getis
end

def ok_arr(arr)
  return false unless arr.size == 9
  h = arr.tally
  (1..9).all? do |i|
     h[i] == 1
  end
end

def ok?(b)
  return false unless b.all? { |e| ok_arr(e) }
  9.times do |i|
    arr = b.map {|e| e[i] }
    return false unless ok_arr(arr)
  end

  [
    [0, 0],
    [0, 3],
    [0, 6],
    [3, 0],
    [3, 3],
    [3, 6],
    [6, 0],
    [6, 3],
    [6, 6],
  ].all? do |dx, dy|
    arr = [
      [0, 0],
      [0, 1],
      [0, 2],
      [1, 0],
      [1, 1],
      [1, 2],
      [2, 0],
      [2, 1],
      [2, 2],
    ].map do |x, y|
      xx = x + dx
      yy = y + dy
      b[xx][yy]
    end

    ok_arr(arr)
  end
end

if ok?(b)
  puts "Yes"
else
  puts "No"
end
