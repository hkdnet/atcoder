def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N = geti
S = gets.chomp

memo = Hash.new do |h, k|
  h[k] = {}
end

solve = ->(l, r) {
  next 0 if l >= r
  memo[l][r] ||=
    begin
      arr = []
      if S[l] == S[r-1]
        d = l == r - 1 ? 1 : 2
        arr << solve[l+1, r-1] + d
      end
      arr << solve[l+1, r]
      arr << solve[l, r-1]
      arr.max
    end
}

ans = solve[0, N]
puts ans
