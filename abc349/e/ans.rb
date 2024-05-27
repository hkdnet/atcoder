def getl = gets.chomp
def geti = getl.to_i
def getis = getl.split(" ").map(&:to_i)
def getis1 = getl.split(" ").map { |e| e.to_i - 1 }

arr = []
arr << getis
arr << getis
arr << getis
board = arr.flatten
pat = [
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  [0, 4, 8],
  [2, 4, 6],
]
memo = {}
msolve = nil
solve = ->(b, mark) {
  pat.each do |pattern|
    fs = pattern.map { |i| b[i] }
    next if fs.any?(&:nil?)

    if fs.all? {|b| b == fs[0] }
      return fs[0]
    end
  end

  if b.none?(&:nil?)
    # 終わり
    t = 0
    a = 0
    b.zip(board) do |f, point|
      if f
        t += point
      else
        a += point
      end
    end
    return t > a
  end

  9.times do |i|
    next unless b[i].nil?

    b[i] = mark
    ret = msolve[b, !mark]
    b[i] = nil
    if ret == mark
      return mark
    end
  end

  !mark
}
msolve = ->(b, mark) {
  if memo.key?(b)
    memo[b]
  else
    memo[b] = solve[b, mark]
  end
}

if msolve[[nil] * 9, true]
  puts "Takahashi"
else
  puts "Aoki"
end
