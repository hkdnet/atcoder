def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end
N = geti

points = N.times.map do |i|
  pi, a = getis
  [pi-1, a]
end

delta = ->(l, r, idx) {
  target, d = points[idx]
  if l <= target && target < r
     d
  else
    0
  end
}
memo = Hash.new do |h, k|
  h[k] = {}
end
# 残りが [l, r) のときの最高スコア
solve = ->(l, r) {
  next 0 if l == r
  memo[l][r] ||=
    begin
      # l を取った場合
      sl = delta[l+1, r, l] + solve[l+1, r]
      # r-1 を取った場合
      sr = delta[l, r-1, r-1] + solve[l, r-1]
      if sl > sr
        sl
      else
        sr
      end
    end
}
ans = solve[0, N]
puts ans
# p memo
