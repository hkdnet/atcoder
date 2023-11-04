def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
ps = getis

dp = Array.new(N+1) { -1200 }

ps.each do |point|
  (0..N).each do |i|
    i = N - i - 1
    if dp[i] == -1200
      next
    end

    dp[i] = dp[i-1]
  end
  0.upto(N-1) do |i|
    new_dp = {}

    dp.each do |selected_count, cur|
      # skip
      skip_score = cur[:score]
      if !new_dp.key?(selected_count) || new_dp[selected_count][:score] < skip_score
        new_dp[selected_count] = dp[selected_count]
      end

      # pick
      nu, nb, pick_score = diff_rate(selected_count + 1, cur[:u], cur[:b], ps[i])
      if !new_dp.key?(selected_count+1) || new_dp[selected_count+1][:score] < pick_score
        new_dp[selected_count+1] = { score: pick_score, u: nu, b: nb }
      end
    end

    dp = new_dp
  end
end

ans = nil
dp.each do |k, v|
  next if k == 0
  ans = v[:score] if ans.nil? || ans < v[:score]
end
puts ans
