def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
memo = {
  1 => 0,
  2 => 2
}

calc = ->(n) {
  memo[n] ||=
    begin
      a, b = n.even? ? [n/2, n/2] : [n/2, n/2 + 1]
      n + calc[a] + calc[b]
    end
}

puts calc[N]
