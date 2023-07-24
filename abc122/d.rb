# frozen_string_literal: true

N = gets.chomp.to_i
MOD = 10**9 + 7

dp = {}
%w[A C G T].each do |c1|
  %w[A C G T].each do |c2|
    %w[A C G T].each do |c3|
      k = c1 + c2 + c3
      dp[k] = 1
    end
  end
end

dp.delete("AGC")
dp.delete("GAC")
dp.delete("ACG")

def next_key(old, c)
  old[1, 2] + c
end

(N-3).times do
  tmp = Hash.new { |h, k| h[k] = 0 }

  dp.each do |k, v|
    %w[A C G T].each do |c|
      next if c == "C" && (k == "ATG" || k == "AGG" || k == "AGT")

      nx = next_key(k, c)
      next if nx == "AGC" || nx == "GAC" || nx == "ACG"

      tmp[nx] += v
    end
  end

  dp = tmp
end

ans = dp.values.sum
puts ans % MOD
