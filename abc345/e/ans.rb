def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, K = getis
balls = N.times.map do
  getis
end

class H < Hash
  def update_if_larger(k, v)
    cur = self[k]
    self[k] = v if cur < v
  end
end

# col -> removed -> sum
def build_dp
  Hash.new do |h, k|
    h[k] = H.new do |hh, kk|
      hh[kk] = -1
    end
  end
end
dp = build_dp
dp[-1][0] = 0

balls.each do |c, k|
  tmp = build_dp
  dp.each do |col, h|
    h.each do |removed, sum|
      # remove
      tmp[col].update_if_larger(removed+1, sum)
      if c != col
        # pick
        tmp[c].update_if_larger(removed, sum + k)
      end
    end
  end

  dp = tmp
end

ans = -1
dp.each do |_col, h|
  ans = h[K] if ans < h[K]
end
puts ans
